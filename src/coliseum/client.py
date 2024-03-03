import os
import sys
import ssl
import socket
import threading

# Adding module to python path
sys.path.append(os.getcwd())

from utils import (
    SERVER_IP,
    SERVER_HOSTNAME,
    SERVER_PORT,
    CA_FILE_PATH,
    Message,
    configure_logging, 
)

logger = configure_logging("thread-client.log")


class Client(threading.Thread):

    def __init__(self):
        self._server_ip = SERVER_IP
        self._server_hostname = SERVER_HOSTNAME
        self._server_port = SERVER_PORT
        
        self.sock: socket.socket = None
        self.id:str = None
    

    def _initial_connection(self):
        
        logger.debug("Initial connection to server")
        
        try:
            # Envio del mensaje de conexión
            message = Message(sender_id=None,status=201,content="new_user")
            self.sock.sendall(message.encode())
            logger.debug("Initial connection meesage send to server")

            # Respuesta del servidor indicando el nombre de usuario
            raw_message = self.sock.recv(1024)
            message:Message = Message.create(raw_message)

            # Si el status es incorrecto cierra la conexión
            if message.status !=200:
                message.print()
                self.sock.close()
                logger.warning("Connection Refused")
                raise

            # Si el status es correcto implica que ha accedido
            # al sistema y el contenido es el ID del cliente
            self.id = message.content
            logger.info(f"Client connected to server with id:{self.id}")
            print(f"You are now online! Your ID: {self.id}")

        except Exception as error:
            logger.error(error)
            raise

    def _send_message(self):
        try:
            while True:
                content = input()
                if content.lower() == 'exit':
                    break
                logger.debug("Sending message to server")
                message = Message(sender_id = self.id,content=content)
                self.sock.sendall(message.encode())
                
        except Exception as error:
            logger.error(error)
            raise

        finally:
            self.sock.close()


    def _recive_message(self):
        try:
            while True:
                raw_message = self.sock.recv(1024)
                if not raw_message:
                    break
                
                logger.debug("Message recive from server")
                message = Message.create(raw_message)
                message.print()
                
        except Exception as error:
            logger.error(error)
            raise



    def run(self):

        logger.debug("Creating socket")

        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
                
                context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)

                # Server authentication
                context.load_verify_locations(cafile=CA_FILE_PATH)

                self.sock = context.wrap_socket(sock=sock, server_hostname=self._server_hostname)

                logger.debug("Connecting to server...")
                self.sock.connect((self._server_ip, self._server_port))
                logger.debug("Conection establisehd with server!")
                
                self._initial_connection()

                logger.debug("Creatng thread for recieving messages")
                receive_thread = threading.Thread(target=self._recive_message)
                receive_thread.start()

                logger.debug("Creating thread for sending messages")
                send_thread = threading.Thread(target=self._send_message)
                send_thread.start()

                # Wait for the send and receive threads to finish
                send_thread.join()
                receive_thread.join()

        except Exception as error:
            logger.error(error)
            raise

if __name__ == "__main__":
    client = Client()
    try:
        client.run()
    except KeyboardInterrupt:
        print("Client terminated by user.")