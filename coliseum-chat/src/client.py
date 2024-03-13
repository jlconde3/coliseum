import os
import sys
import asyncio

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


class Client:
    
    def __init__(self) -> None:
        self.reader: asyncio.StreamReader
        self.writer: asyncio.StreamWriter

    async def initial_connection(self):
        """Función empleada para la conexión inicial al servidor.
        Con al intención de recuperar los datos necesarios para el correcto uso desde el cliente."""

        # Envio del mensaje de conexión
        message = Message(sender_id="",status=201,content="new_user")
        self.writer.write(message.encode())

        # Respuesta del servidor indicando el id de usuario
        raw_message = await self.reader.read(1024)
        message:Message = Message.create(raw_message)
        self.id = message.content
        logger.info(f"Client connected to server with id:{self.id}")
        message.content = f"You are now online! Your ID: {self.id}"
        print(message.content)


    async def send_message(self):
        """Send a message to the server"""
        
        input_text = input(">>> ") # El problema es que el INPUT bloque la visualziación
        message = Message(content=input_text)
        self.writer.write(message.encode())
        await self.writer.drain()
        print(f"Sended message to server: {message.content}")

    async def receive_message(self):
        raw_message = await self.reader.read(1024)
        message:Message = Message.create(raw_message)
        print(f"Received message from server: {message.content}")

    async def run(self):
        try:

            self.reader, self.writer = await asyncio.open_connection(SERVER_IP, SERVER_PORT)
            await self.initial_connection()

            send_task = asyncio.create_task(self.send_message())
            recive_task = asyncio.create_task(self.receive_message())
            
            await send_task
            await recive_task

        except Exception as error:
            print(error)
        
        finally:
            self.writer.close()



# Replace SERVER_IP and SERVER_PORT with your actual server details

if __name__ == "__main__":
# Create an instance of the Client class and run the event loop
    client = Client()
    asyncio.run(client.run())


            

