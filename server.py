import socket
import json
import threading

HOST = "127.0.0.1"
PORT = 65432

class ClientHandler(threading.Thread):
    def __init__(self, conn: socket.socket, addr):
        super().__init__()
        self.conn = conn
        self.addr = addr

    def run(self):
        try:
            while True:
                data = self.conn.recv(1024)
                if not data:
                    break

                received_data = json.loads(data.decode("utf-8"))
                self.broadcast(received_data)
                print(f"Received and broadcast data from {self.addr}")

        except Exception as e:
            print(f"Error handling connection with {self.addr}: {e}")

        finally:
            # Close the connection only if it's not already closed
            if not self.conn._closed:
                print(f"Closing connection:{self.addr}")
                self.conn.close()

            clients.remove(self)

    def broadcast(self, data):
        for client in clients:
            if client != self:
                try:
                    client.conn.sendall(json.dumps(data).encode("utf-8"))
                except:
                    # Handle any potential exceptions when sending data
                    pass

clients:list[ClientHandler] = []

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))
    s.listen()

    print(f"Server listening on {HOST}:{PORT}")

    while True:
        conn, addr = s.accept()
        client_handler = ClientHandler(conn, addr)
        clients.append(client_handler)
        client_handler.start()
