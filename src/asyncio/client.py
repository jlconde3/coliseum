import asyncio
import os
import sys
import ssl

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

logger = configure_logging("async-client.log")


class Client:
    def __init__(self):
        self._server_ip = SERVER_IP
        self._server_hostname = SERVER_HOSTNAME
        self._server_port = SERVER_PORT
        self.id = None
        self.reader = None
        self.writer = None

    async def _initial_connection(self):
        try:
            # Respuesta del servidor indicando el nombre de usuario
            raw_message = await self.reader.read(1024)
            message = Message.create(raw_message)

            # Si el status es incorrecto cierra la conexión
            if message.status != 200:
                message.print()
                self.writer.close()
                logger.warning("Connection Refused")
                raise ConnectionRefusedError()

            self.id = message.content
            logger.info(f"Client connected to server with id: {self.id}")
            print(f"You are now online! Your ID: {self.id}")

        except Exception as error:
            logger.error(error)
            raise

    async def _send_message(self):
        try:
            while True:
                content = input()
                if content.lower() == 'exit':
                    break
                message = Message(sender_id=self.id, content=content)
                self.writer.write(message.encode())
                await self.writer.drain()
                logger.debug("Message sent to server")

        except Exception as error:
            logger.error(error)
            raise

    async def _receive_message(self):
        try:
            while True:
                raw_message = await self.reader.read(1024)
                if not raw_message:
                    break
                logger.debug("Message received from server")
                message = Message.create(raw_message)
                message.print()

        except Exception as error:
            logger.error(error)
            raise

    async def run(self):
        try:
            context = ssl.create_default_context(ssl.Purpose.SERVER_AUTH, cafile=CA_FILE_PATH)

            self.reader, self.writer = await asyncio.open_connection(
                host=self._server_ip,
                port=self._server_port,
                ssl=context,
                server_hostname=self._server_hostname,
            )
            await self._initial_connection()

            send_task = asyncio.create_task(self._send_message())
            receive_task = asyncio.create_task(self._receive_message())

            logger.debug("Creating tasks for receiving and sending messages")
            await asyncio.gather(receive_task, send_task)

        except Exception as error:
            logger.error(error)
            raise

        finally:
            if self.writer and not self.writer.is_closing():
                self.writer.close()
                await self.writer.wait_closed()
                logger.info("Connection closed")

if __name__ == "__main__":
    try:
        client = Client()
        asyncio.run(client.run())
    except KeyboardInterrupt:
        print("Client terminated by user.")

