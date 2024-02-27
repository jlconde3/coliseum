
import os
import socket
import threading


from config import logger
from utils import Message

# Carga de las variables de entorno

    

class Client(threading.Thread):

    def __init__(self):
        self.conn: socket.socket = None
        self.addr:tuple = ()

        self._host = self.addr[0]
        self._port = self.addr[1]

        self.server_host = os.getenv("SERVER_HOST")
        self.server_port = int(os.getenv("SERVER_PORT"))
        self.id:str = None
    

    def _initial_connection(self):
        
        logger.debug("Initial connection to server")
        
        try:
            # Envio del mensaje de conexión
            message = Message(None,"new_user")
            self.conn.sendall(message.encode())
            logger.debug("Connection Message send to server")

            # Respuesta del servidor indicando el nombre de usuario
            raw_message = self.conn.recv(1024)
            message:Message = Message.create(raw_message)

            # Si el status es incorrecto cierra la conexión
            if message.status !=200:
                message.print()
                self.conn.close()
                logger.warning("Connection Refused")
                raise ConnectionRefusedError()
        
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
                message = Message(self._host,self.id,content)
                self.conn.sendall(message.encode())
        
        except Exception as error:
            logger.error(error)
            raise


    def _recive_message(self):

        try:
            while True:
                raw_message = self.conn.recv(1024)
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

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
            
            self.conn, self.addr = sock

            logger.debug("Connecting to server...")
            sock.connect((self.server_host, self.server_port))
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


if __name__ == "__main__":
    client = Client()

    try:
        client.run()
    except KeyboardInterrupt:
        print("Client terminated by user.")