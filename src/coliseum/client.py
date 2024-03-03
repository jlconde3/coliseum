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


class Client(threading.Thread):

    def __init__(self, root:tk.Tk):

        self.id:str
        self.sock: socket.socket

        self._server_ip = SERVER_IP
        self._server_hostname = SERVER_HOSTNAME
        self._server_port = SERVER_PORT

        self.root = root
        self.client = client

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

        message = Message(
            sender_id="",
            content=input_text,
        )

        # Limpia la entrada después de presionar Enter
        self.input_entry.delete(0, tk.END)
        self.input_entry.insert(tk.END, ">>> ")



        # Desplaza automáticamente hacia abajo para mostrar el mensaje más reciente
        self.text_widget.yview(tk.END)


    def _initial_connection(self):

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
                self.print_server_message(message)
                logger.warning("Connection Refused")
                raise

            # Si el status es correcto implica que ha accedido
            # al sistema y el contenido es el ID del cliente
            self.id = message.content
            logger.info(f"Client connected to server with id:{self.id}")
            message.content = f"You are now online! Your ID: {self.id}"
            self.print_server_message(message)

        except Exception as error:
            logger.error(error)
            raise


    def _send_message(self):
        """Lógica para el envío de un mensaje al servidor desde el cliente"""
        try:
            while True:
                content = self.input_entry.get()
                if content == "exit":
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
        """Lógica para recivir mensajes desde el servidor y mostrarlos en el cliente"""
        try:
            while True:
                raw_message = self.sock.recv(1024)
                if not raw_message:
                    break
                logger.debug("Message recive from server")
                message = Message.create(raw_message)
                self.print_server_message(message)
        except Exception as error:
            logger.error(error)
            raise
        finally:
            self.sock.close()


    def run(self):

        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:

                # SSL/TLS Server authentication disabled in DEBUG Mode
                #context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
                #context.load_verify_locations(cafile=CA_FILE_PATH)
                #self.sock = context.wrap_socket(sock=sock, server_hostname=self._server_hostname)

                self.sock = sock
                logger.debug("Connecting to server...")
                self.sock.connect((self._server_ip, self._server_port))
                logger.debug("Conection establisehd with server!")

                self._initial_connection()

                logger.debug("Creating thread for recieving messages")
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






class ClientGUI:
    def __init__(self, root, client:Client):
        self.root = root
        self.client = client
        self.root.title("Terminal App")
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

        self.input_entry.focus_set()

    def process_input(self, event):
        # Obtiene el texto de la entrada
        input_text = self.input_entry.get()

        message = Message(
            sender_id="",
            content=input_text,
        )

        # Limpia la entrada después de presionar Enter
        self.input_entry.delete(0, tk.END)
        self.input_entry.insert(tk.END, ">>> ")

        self.text_widget.config(state=tk.NORMAL)
        self.text_widget.insert(tk.END, f"{message.created_time}/{message.sender_id}:{message.content}")
        self.text_widget.config(state=tk.DISABLED)

        # Desplaza automáticamente hacia abajo para mostrar el mensaje más reciente
        self.text_widget.yview(tk.END)




if __name__ == "__main__":
    root = tk.Tk()
    client = Client()
    try:
        client.run()
    except KeyboardInterrupt:
        print("Client terminated by user.")
