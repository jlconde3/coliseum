
import os
import ssl
import socket
import threading

from utils import configure_logging
from messages import Message


class Client(threading.Thread):

    def __init__(self):

        self.sock: socket.socket = None
        self.server_host = os.getenv("SERVER_HOST")
        self.server_port = int(os.getenv("SERVER_PORT"))
        self.id:str = None
    

    def _initial_connection(self):
        
        logger.debug("Initial connection to server")
        
        try:
            # Envio del mensaje de conexión
            message = Message(sender_id=None,status=201,content="new_user")
            self.sock.sendall(message.encode())
            logger.debug("Connection Message send to server")

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
                context.load_cert_chain(certfile="ca.pem")

                self.sock = context.wrap_socket(sock=sock,server_hostname=self.server_host)

                logger.debug("Connecting to server...")
                self.sock.connect((self.server_host, self.server_port))
                logger.debug("Conection establisehd with server!")

                self._initial_connection()

                logger.debug("Thread for recieving messages")
                receive_thread = threading.Thread(target=self._recive_message)
                receive_thread.start()

                logger.debug("Thread for sending messages")
                send_thread = threading.Thread(target=self._send_message)
                send_thread.start()

                # Wait for the send and receive threads to finish
                send_thread.join()
                receive_thread.join()
        except:
            pass

if __name__ == "__main__":
    
    logger = configure_logging("client.log")
    client = Client()
    try:
        client.run()
    except KeyboardInterrupt:
        print("Client terminated by user.")