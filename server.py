import socket
import json
import threading

from dataclasses import dataclass


"""
1- Pantalla de inicio
2- Presiona ENTER para conectar contra el servicio
3 - El servidor comprueba el número de usuario en línea, si es superior a 10
    - El nuevo usuario asume el rol de espectador, donde solo puede ver los mensajes.
    - Si es menor a 10, asigna el rol de jugador. Devuelve un id y el número de usuarios en línea.

4- El cliente escribe un mensaje y lo envia presionando enter.
5- El servidor recive el mensaje
6- El servidor envia el mensaje a los usuarios en linea.
7- Se repiten los paso 4,5,6
8- Para salir del chat, usuario presiona Ctrl + C
9- El servidro debe ser de asignar el rol espectador a los clientes que lleven menos de 10 minutos si escribir un mensaje.
"""



import socket
import threading
import json

@dataclass
class Message:
    sender: str
    content: str

    def to_json(self):
        return json.dumps(self.__dict__).encode("utf-8")


def create_message(message: bytes):
    """Factory pattern to create a message object"""
    try:
        data: dict = json.loads(message.decode("utf-8"))
        sender = data.get("sender")
        content = data.get("content")
        return Message(sender=sender, content=content)
    except json.JSONDecodeError:
        # Handle JSON decoding errors
        print("Error decoding JSON message")
        return None


class Server:
    def __init__(self):
        self._host = "127.0.0.1"
        self._port = 65432
        self.clients: list[ClientHandler] = []
        self._lock = threading.Lock()
        self._client_counter = 0  # Counter for generating unique client IDs

    def run(self):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind((self._host, self._port))
            s.listen()

            print(f"Server listening on {self._host}:{self._port}")

            try:
                while True:
                    conn, addr = s.accept()
                    with self._lock:
                        # Si hay más de dos usuarios no permite la conexión
                        if len(self.clients) >= 2:
                            # Informar al cliente que la conexión ha sido denegada
                            rejection_message = Message(sender="Server", content="Connection rejected. Server is full.")
                            conn.sendall(rejection_message.to_json())
                            conn.close()
                        else:
                            client_handler = ClientHandler(conn, addr, self, self._get_next_client_id())
                            self.clients.append(client_handler)
                            self._broadcast_online_clients()
                            client_handler.start()

            except KeyboardInterrupt:
                print("Server shutting down.")
            
    def remove_client(self, client_handler):
        with self._lock:
            self.clients.remove(client_handler)
            self._broadcast_online_clients()

    def _get_next_client_id(self):
        with self._lock:
            self._client_counter += 1
            return self._client_counter

    def _broadcast_online_clients(self):
        message = Message(sender="Server", content=f"Online clients: {len(self.clients)}")
        for client in self.clients:
            try:
                client.conn.sendall(message.to_json())
            except Exception as e:
                print(f"Error broadcasting online clients message: {e}")
                client.disconnect()


class ClientHandler(threading.Thread):
    def __init__(self, conn: socket.socket, addr, server: Server):
        super().__init__()
        self.conn = conn
        self.addr = addr
        self.server = server

    def run(self):
        try:
            while True:
                raw_message = self.conn.recv(1024)
                if not raw_message:
                    break

                message = create_message(raw_message)
                if message:
                    self.broadcast(message)

        except Exception as e:
            print(f"Error in client handler: {e}")
        finally:
            self.disconnect()

    def disconnect(self):
        if not self.conn._closed:
            self.conn.close()
            self.server.remove_client(self)

    def broadcast(self, message: Message):
        with self.server._lock:
            for client in self.server.clients:
                if client == self:
                    continue
                try:
                    client.conn.sendall(message.to_json())
                except Exception as e:
                    print(f"Error broadcasting to client: {e}")
                    client.disconnect()


if __name__ == "__main__":
    server= Server()

    server.run()