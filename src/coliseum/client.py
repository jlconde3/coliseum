
import os
import socket
import json
import threading
import logging


from dotenv import find_dotenv, load_dotenv
from dataclasses import dataclass

# Carga de las variables de entorno

path = find_dotenv()
if path:
    load_dotenv()


def configure_logging():
    # Logging manager configuration
    logger = logging.getLogger("coliseum_client")
    logger.setLevel(logging.DEBUG)
    fh = logging.FileHandler(f"coliseum_client.log")
    formatter = logging.Formatter('%(asctime)s - %(name)s- %(levelname)s - %(message)s')
    fh.setFormatter(formatter)
    logger.addHandler(fh)
    return logger


    

class Client(threading.Thread):

    
    def __init__(self):
        self._host = os.getenv("SERVER_HOST")
        self._port = int(os.getenv("SERVER_PORT"))
        self.sock:socket.socket = None
        self.user_id:str = None

    @staticmethod
    def stdout_message(message:Message):
        print(f"{message.sender}:{message.content}")

    def _initial_connection(self):
        
        logger.debug("Initial connection to server")
        
        try:
            # Envio del mensaje de conexión
            message = Message(None,"new_user")
            self.sock.sendall(message.to_bytes())
            logger.debug("Connection Message send to server")

            # Respuesta del servidor indicando el nombre de usuario
            response = self.sock.recv(1024)
            message:Message = create_message(response)
            self.stdout_message(message)
            self.user_id = message.content
            
            logger.info(f"Client connected to server with id:{self.user_id}")
            print(f"You are now online! Your ID: {self.user_id}")

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
                message = Message(self.user_id,content)
                self.sock.sendall(message.to_bytes())
        
        except Exception as error:
            logger.error(error)
            raise


    def _recive_message(self):

        try:
            while True:
                message_bytes = self.sock.recv(1024)
                if not message_bytes:
                    break
                
                logger.debug("Message recive from server")
                message = create_message(message_bytes)
                self.stdout_message(message)
                
        except Exception as error:
            logger.error(error)
            raise



    def run(self):

        logger.debug("Creating socket")

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:

            self.sock:socket.socket = sock

            logger.debug("Connecting to server...")
            sock.connect((self._host, self._port))
            logger.debug("Conection establisehd with server!")

            print(initial_message)
                        
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


if __name__ == "__main__":
    logger = configure_logging()
    client = Client()

    try:
        client.run()
    except KeyboardInterrupt:
        print("Client terminated by user.")