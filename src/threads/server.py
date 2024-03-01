import os
import ssl
import socket
import threading

from utils import configure_logging, Message

class Server:
    """Class representing a simple server."""

    def __init__(self):
        """Initialize the server."""
        self._host = os.getenv("SERVER_HOST")
        self._port = int(os.getenv("SERVER_PORT"))

        self._lock = threading.Lock()
        self._client_counter = 0  # Counter for generating unique client IDs
        self.clients: list[ClientHandler] = []

    @staticmethod
    def _send_reject_message(conn: socket.socket):
        """Send rejection message to the client."""
        try:
            message = Message(sender_id="Server", status=500, content="server_full")
            conn.sendall(message.encode())
        except socket.error as error:
            logger.error(error)
            raise
        finally:
            conn.close()

    def _get_len_clients(self):
        """Get the length of server clients."""
        logger.debug("STACK: Get length from server.clients")
        try:
            with self._lock:
                len_clients = len(self.clients)  # Use len() function to get the length
            return len_clients
        except Exception as error:
            logger.error(error)
            raise

    def _append_client(self, client_handler):
        """Append a client to server.clients."""
        logger.debug("STACK: Append client to server.clients")
        try:
            with self._lock:
                self.clients.append(client_handler)
        except Exception as error:
            logger.error(error)
            raise

    def _get_next_client_id(self):
        """Get the next client ID."""
        logger.debug("STACK: Get the next client ID")
        try:
            with self._lock:
                self._client_counter += 1
            return self._client_counter
        except Exception as error:
            logger.error(error)
            raise

    def _broadcast_user_id(self, client_handler: 'ClientHandler'):
        """Broadcast user ID to the client."""
        logger.info("Sending user id to client")
        try:
            message = Message(sender_id="Server", content=client_handler.id)
            client_handler.conn.sendall(message.encode())
        except Exception as error:
            logger.error(error)
            raise

    def _remove_disconnected_clients(self):
        """Remove disconnected clients from server.clients."""
        with self._lock:
            self.clients = [client for client in self.clients if not client.conn._closed]


    def remove_client(self, client_handler: 'ClientHandler'):
        """Remove a client from server.clients."""
        logger.debug("STACK: Remove client from server.clients")
        try:
            with self._lock:
                self.clients.remove(client_handler)
        except Exception as error:
            logger.error(error)
            raise

    def run(self):
        """Run the server."""
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:

            context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
            # Client authentication
            context.load_cert_chain(certfile=r".\certs\ca.pem", keyfile=r".\certs\ca.key")
            sock = context.wrap_socket(sock=sock,server_side=True)
            
            try:
                sock.bind((self._host, self._port))
        
            except Exception as error:
                logger.error(error)

            sock.listen()

            logger.info(f"Server listening on: {self._host}:{self._port}")
            print(f"Server listening on {self._host}:{self._port}")

            try:
                while True:
                    conn, addr = sock.accept()

                    logger.info(f"New connection from: {addr[0]}:{addr[1]}")
                    print(f"New connection from: {addr[0]}:{addr[1]}")

                    if self._get_len_clients() < 3:
                        client_handler = ClientHandler(conn, addr, self)
                        client_handler.id = self._get_next_client_id()
                        self._append_client(client_handler)
                        self._broadcast_user_id(client_handler)
                        client_handler.start()
                    else:
                        logger.warning("Reject connection to client. Server is full")
                        Server._send_reject_message(conn)

            except KeyboardInterrupt:
                logger.info("Server shutting down.")
                print("Server shutting down.")


class ClientHandler(threading.Thread):
    """Class representing a client handler thread."""

    def __init__(self, conn: socket.socket, addr, server: Server, id: int = None):
        """Initialize the client handler."""
        super().__init__()
        self.conn = conn
        self.addr = addr
        self.server = server
        self.id = id

        self._stop_event = threading.Event()

    def _broadcast(self, message: Message):
        """Broadcast a message to server clients."""
        logger.debug("STACK: Broadcasting message to server.clients")

        with self.server._lock:
            connected_clients = [client for client in self.server.clients if not client.conn._closed]

            for client in connected_clients:
                if client == self:
                    continue
                try:
                    client.conn.sendall(message.encode())
                
                except Exception as error:
                    logger.error(error)

        return None
    
    def stop(self):
        self._stop_event.set()

    def disconnect(self):
        """Disconnect the client."""
        if not self.conn._closed:
            self.conn.close()
            self.stop()
            self.server.remove_client(self)
            logger.warning("Client disconnected")
        

    def run(self):
        """Run the client handler."""
        logger.info("Running new client")
        try:
            while not self._stop_event.is_set():
                raw_message = self.conn.recv(1024)

                if not raw_message:
                    break

                message = Message.create(raw_message)
                self._broadcast(message)

        except Exception as error:
            logger.error(error)

        finally:
            self.disconnect()



if __name__ == "__main__":
    
    logger = configure_logging("server.log")
    server= Server()
    server.run()