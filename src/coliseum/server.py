import os
import socket
import threading

from utils import configure_logging
from messages import Message



class Server:
    def __init__(self):
        self._host = os.getenv("SERVER_HOST")
        self._port = int(os.getenv("SERVER_PORT"))

        self._lock = threading.Lock()
        self._client_counter = 0  # Counter for generating unique client IDs
        self.clients: list[ClientHandler] = []

    
    def _send_reject_message(client_handler:'ClientHandler'):
        try:
            message = Message(sender_id="Server", status=500,content="server_full")
            client_handler.conn.sendall(message.encode())
        except Exception as error:
            logger.error(error)
            raise
        finally:
            client_handler.conn.close()

    def _get_len_clients(self):
        logger.debug("STACK: Get length from server.clients")
        try:
            with self._lock:
                len_clients = self.clients
            return len(len_clients)
        except Exception as error:
            logger.error(error)
            raise
        
    def _append_client(self,client_handler):
        logger.debug("STACK: Append client to server.clients")
        try:
            with self._lock:
                self.clients.append(client_handler)
        except Exception as error:
            logger.error(error)
            raise
    
    def _get_next_client_id(self):
        logger.debug("STACK: Get server.client_counter")
        try:
            with self._lock:
                self._client_counter += 1
            return self._client_counter
        except Exception as error:
            logger.error(error)
            raise
    
    def _broadcast_user_id(self,client_handler:'ClientHandler'):
        logger.info("Sending user id to client")
        try:
            message = Message(sender_id="Server", content=client_handler.id)
            client_handler.conn.sendall(message.encode())
        except Exception as error:
            logger.error(error)
            raise
    
    def remove_client(self, client_handler:'ClientHandler'):
        logger.debug("STACK: Remove client from server.clients")
        try:
            with self._lock:
                self.clients.remove(client_handler)
        except Exception as error:
            logger.error(error)
            raise

    def run(self):

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind((self._host, self._port))
            s.listen()

            logger.info(f"Server listening on: {self._host}:{self._port}")
            print(f"Server listening on {self._host}:{self._port}")

            try:
                while True:
                    conn, addr = s.accept()

                    logger.info(f"New connection from: {addr[0]}:{addr[1]}")
                    print(f"New connection from: {addr[0]}:{addr[1]}")
                    
                    client_handler = ClientHandler(conn, addr, self)

                    if self._get_len_clients()<=2:
                        client_handler.id = self._get_next_client_id()
                        self._append_client(client_handler)
                        self._broadcast_user_id(client_handler)
                        client_handler.start()
                    else:
                        logger.warning("Reject connection to client. Server is full")
                        Server._send_reject_message(client_handler)
                    

            except KeyboardInterrupt:
                logger.info("Server shutting down.")
                print("Server shutting down.")



class ClientHandler(threading.Thread):

    def __init__(self, conn: socket.socket, addr, server: Server, id:int=None):
        super().__init__()
        self.conn = conn
        self.addr = addr
        self.server = server
        self.id = id


    def _broadcast(self, message: Message):
        logger.debug("STACK: Broadcasting message to server.clients")

        if not message.sender_id:
            return None

        with self.server._lock:
            for client in self.server.clients:
                if client == self:
                    continue
                try:
                    client.conn.sendall(message.encode())
                except Exception as error:
                    logger.error(error)
                    client.disconnect()
                    break

        return None
    
    def disconnect(self):
        if not self.conn._closed:
            self.conn.close()
            self.server.remove_client(self)

    def run(self):
        logger.info("Running new client")
        try:
            while True:
                raw_message = self.conn.recv(1024)
                if not raw_message:
                    break
                message = Message.create(raw_message)

                if message:
                    self._broadcast(message)

        except Exception as error:
            logger.error(error)
            self.disconnect()


if __name__ == "__main__":
    
    logger = configure_logging("server.log")
    server= Server()
    server.run()