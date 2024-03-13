import time
import uuid
import threading
import argparse

import requests
from flask import Flask, request, jsonify

class Node:
    def __init__(self, host, port, register_host):
        self.host = host
        self.port = port
        self.register_node = register_host
        self.connected_nodes:set[str] = set()
        self.lock = threading.Lock()
        self.data = {} # En memoría se guardan los datos distribuidos por los nodos.

    def register_with_register_node(self):
        """Implementa la lógica para registrarse con el nodo 'registador'
        en la fase incial de conexión a la red"""
        response = requests.post(f"{self.register_node}/register/node")
        if response.status_code == 200:
            print("Node register successfully")
            self.connected_nodes = response.json().get("nodes")
        else:
            print("Node not register successfully")

    def update_node_list(self, new_node):
        """Añade un nuevo nodo en la red a la lista de nodos"""
        with self.lock:
            self.connected_nodes.add(new_node)

    def distribute_data_to_nodes(self, data: dict):
        """Distribuye los datos a todos los nodos conectados"""
        with self.lock:
            nodes = list(self.connected_nodes)  # Hacer una copia para evitar problemas de iteración y modificación

        for node in nodes:
            headers = {
                "Content-Type": "application/json"
            }
            try:
                response = requests.post(f"{node}/node/receive_data", headers=headers, json=data, timeout=10)
                response.raise_for_status()  # Generará una excepción si la respuesta HTTP indica un error
                print(f"Data successfully sent to Node {node}")
            except requests.exceptions.RequestException as error:
                # Si ocurre un error, manejarlo y actualizar la lista de nodos conectados
                print(f"Error sending data to Node {node}: {error}")
                with self.lock:
                    self.connected_nodes.discard(node)  # Usar discard para evitar KeyError si el nodo ya fue eliminado

    def check_node_connectivity(self):
        while True:
            with self.lock:
                nodes = self.connected_nodes

            for node in list(nodes):
                if not self.is_node_reachable(node):
                    self.connected_nodes.remove(node)
                    print(f"Node {node} disconnected and removed from the list.")
            time.sleep(60)  # Verifica la conectividad cada 60 segundos

    def is_node_reachable(self, node):
        """Contacta con un nodo al endpoint 'test'"""
        response = requests.get(f"{node}/test")
        if response.status_code ==200:
            return True
        return False  # Reemplaza esto con tu propia lógica de verificación
    
    def handle_data(self,data):
        """Gestiona los datos recividos por un cliente o nodo"""
        with self.lock:
            self.data.update({str(uuid.uuid4()):data})

    def handle_client(self, data:dict):
        """Gestiona el contacto de un cliente con self y con los datos que envía"""
        self.handle_data(data)
        self.distribute_data_to_nodes(data)

    def handle_node(self, data:dict):
        """Gestiona el contacto de un nodo con self y los datos que envía"""
        self.handle_data(data)

# Cambiar a protocolo TCP
    def start_server(self):
        app = Flask(__name__)

        @app.route('/test', methods = ["GET"])
        def test_connection():
            return jsonify({"message": "Node online"}), 200
        
        @app.route('/register/node', methods=["PUT"])
        def register_node():
            new_node = request.remote_addr
            self.update_node_list(new_node)
            return jsonify({"nodes": self.connected_nodes})

        @app.route('/node/register', methods=["PUT"])
        def node_register():
            new_node = request.remote_addr
            self.update_node_list(new_node)
            return jsonify({"message": "Registration successful"})
        
        @app.route('/client/receive_data', methods=['POST'])
        def client_receive_data():
            data = request.get_json()
            self.handle_client(data)
            return jsonify({"message": "Data received and distributed successfully"})
        
        @app.route('/node/recive_data', methods=["POST"])
        def node_recive_data():
            data = request.get_json()
            self.handle_node(data)
            return jsonify({"message": "Registration successful"})
        
        threading.Thread(target=app.run, kwargs={'host': self.host, 'port': self.port, "debug":True}).start()

        # Inicia el hilo para comprobar la conectividad con los nodos
        threading.Thread(target=self.check_node_connectivity).start()



def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(description="Distribute system")
    parser.add_argument("-r", "--register", action="store_true", help="Register node")
    parser.add_argument("port", nargs="?", default=5000, help="Port used by node")
    return parser.parse_args()

if __name__ == "__main__":
    args = parse_args()

    node = Node("0.0.0.0", int(args.port), "http://127.0.0.1:5000")

    if not args.register:  # Fix the typo here
        node.register_with_register_node()

    target=node.start_server()