import os
import sys
import ssl
import socket
import threading
import tkinter as tk

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

COLISEUM_LOGO = r"""
 ________  ________  ___       ___  ________  _______   ___  ___  _____ ______
|\   ____\|\   __  \|\  \     |\  \|\   ____\|\  ___ \ |\  \|\  \|\   _ \  _   \
\ \  \___|\ \  \|\  \ \  \    \ \  \ \  \___|\ \   __/|\ \  \\\  \ \  \\\__\ \  \
 \ \  \    \ \  \\\  \ \  \    \ \  \ \_____  \ \  \_|/_\ \  \\\  \ \  \\|__| \  \
  \ \  \____\ \  \\\  \ \  \____\ \  \|____|\  \ \  \_|\ \ \  \\\  \ \  \    \ \  \
   \ \_______\ \_______\ \_______\ \__\____\_\  \ \_______\ \_______\ \__\    \ \__\
    \|_______|\|_______|\|_______|\|__|\_________\|_______|\|_______|\|__|     \|__|
                                      \|_________|
"""


class ClientModel(threading.Thread):

    def __init__(self):

        self.id:str
        self.sock: socket.socket

        # Configuración para la conexión
        self._server_ip = SERVER_IP
        self._server_hostname = SERVER_HOSTNAME
        self._server_port = SERVER_PORT


    def initial_connection(self):

        try:
            # Envio del mensaje de conexión
            message = Message(sender_id="",status=201,content="new_user")
            self.sock.sendall(message.encode())

            # Respuesta del servidor indicando el nombre de usuario
            raw_message = self.sock.recv(1024)
            message:Message = Message.create(raw_message)

            # Si el status es incorrecto cierra la conexión
            if message.status !=200:
                self.sock.close()
                logger.warning("Connection Refused")
                raise

            # Si el status es correcto implica que ha accedido
            # al sistema y el contenido es el ID del cliente
            self.id = message.content
            logger.info(f"Client connected to server with id:{self.id}")
            message.content = f"You are now online! Your ID: {self.id}"

        except Exception as error:
            logger.error(error)
            raise


    def send_message(self, message:Message)->Message:
        """Lógica para el envío de un mensaje al servidor desde el cliente"""
        try:
            self.sock.sendall(message.encode())
            return message
        
        except Exception as error:
            self.sock.close()
            logger.error(error)
            raise


    def recive_message(self, raw_message:bytes)->Message:
        """Lógica para recivir mensajes desde el servidor y mostrarlos en el cliente"""
        try:
            message = Message.create(raw_message)
            return message
        
        except Exception as error:
            self.sock.close()
            logger.error(error)
            raise




class ClientController:
    
    def __init__(self, client:ClientModel) -> None:
        self.client = client


    def send_message(self, message_content:str):
        while True:
            if message_content == "exit":
                break
            message = Message(sender_id=self.client.id, content=message_content)
            self.client.send_message(message)


    def recive_message(self):
        while True:
            raw_message = self.client.sock.recv(1024)
            if not raw_message:
                break
            message = self.client.recive_message(raw_message)


    def run(self):
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:

                # SSL/TLS Server authentication disabled in DEBUG Mode
                #context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
                #context.load_verify_locations(cafile=CA_FILE_PATH)
                #self.sock = context.wrap_socket(sock=sock, server_hostname=self._server_hostname)

                self.sock = sock
                logger.debug("Connecting to server...")
                self.sock.connect((self.client._server_ip, self.client._server_port))
                logger.debug("Conection establisehd with server!")

                self.client.initial_connection()

                logger.debug("Creating thread for recieving messages")
                receive_thread = threading.Thread(target=self.recive_message)
                receive_thread.start()

                logger.debug("Creating thread for sending messages")
                send_thread = threading.Thread(target=self.send_message)
                send_thread.start()

                # Wait for the send and receive threads to finish
                send_thread.join()
                receive_thread.join()

        except Exception as error:
            logger.error(error)
            raise



class ClientView:
    
    def __init__(self, root:tk.Tk, controller:ClientController) -> None:

        self.root = root
        self.controller = controller

        # Configuración para la interfaz
        self.root.title("Coliseum")
        self.root.configure(bg="black")

        # Área de visualización de mensajes con el logotipo
        self.text_widget = tk.Text(root, state=tk.DISABLED, border=-2, relief="flat", bg="black", fg="white", highlightthickness=0)
        self.text_widget.grid(row=0, column=0, sticky=tk.NSEW, padx=5, pady=5)

        # Muestra el mensaje en la zona de visualización
        self.text_widget.config(state=tk.NORMAL)
        self.text_widget.insert(tk.END, COLISEUM_LOGO)
        self.text_widget.insert(tk.END, "\n")
        self.text_widget.config(state=tk.DISABLED)

         # Área de entrada de texto con texto predeterminado ">>>"
        self.input_entry = tk.Entry(root, border=-2, relief="flat", bg="black", fg="white", highlightthickness=0, insertbackground="white")
        self.input_entry.grid(row=1, column=0, sticky=tk.EW, padx=5, pady=5)
        self.input_entry.insert(tk.END, ">>> ")
        self.input_entry.bind("<Return>", self.process_input)

        # Configuración de la geometría de la ventana
        self.root.grid_rowconfigure(0, weight=1)
        self.root.grid_rowconfigure(1, weight=0)
        self.root.grid_columnconfigure(0, weight=1)

        # Cursor en la zona de introducción de datos.
        self.input_entry.focus_set()

            
    def print_message(self,  message:Message):
        self.text_widget.config(state=tk.NORMAL)
        self.text_widget.insert(tk.END, f"{message.created_time}/{message.sender_id}:{message.content}")
        self.text_widget.config(state=tk.DISABLED)


    def process_input(self, event):
        # Obtiene el texto de la entrada
        input_text = self.input_entry.get()
        self.controller.send_message(input_text)

        # Limpia la entrada después de presionar Enter
        self.input_entry.delete(0, tk.END)
        self.input_entry.insert(tk.END, ">>> ")

        # Desplaza automáticamente hacia abajo para mostrar el mensaje más reciente
        self.text_widget.yview(tk.END)


if __name__ == "__main__":
    root = tk.Tk()
    client = Client(root)
    try:
        client.run()
    except KeyboardInterrupt:
        print("Client terminated by user.")
