
import asyncio
import os
import sys
import ssl
from typing import List

# Adding module to python path
sys.path.append(os.getcwd())


from utils import (
    SERVER_IP,
    SERVER_HOSTNAME,
    SERVER_PORT,
    CA_FILE_PATH,
    KEY_FILE_PATH,
    Message,
    configure_logging,
)

logger = configure_logging("async-server.log")


class Server:
    """Class representing a simple server."""

    def __init__(self):
        """Initialize the server."""
        self._server_ip = SERVER_IP
        self._server_hostname = SERVER_HOSTNAME
        self._server_port = SERVER_PORT
        self._client_counter = 0  # Counter for generating unique client IDs
        self.clients: List[ClientHandler] = []

    async def _send_reject_message(self, writer: asyncio.StreamWriter):
        """Send rejection message to the client."""
        try:
            message = Message(sender_id="Server", status=500, content="server_full")
            writer.write(message.encode())
            await writer.drain()
        except Exception as error:
            logger.error(error)
            raise
        finally:
            writer.close()

    def _get_len_clients(self):
        """Get the length of server clients."""
        return len([client for client in self.clients if not client.is_closed()])

    def _append_client(self, client_handler):
        """Append a client to server.clients."""
        self.clients.append(client_handler)

    def _get_next_client_id(self):
        """Get the next client ID."""
        self._client_counter += 1
        return self._client_counter

    async def _broadcast_user_id(self, client_handler: 'ClientHandler'):
        """Broadcast user ID to the client."""
        try:
            message = Message(sender_id="Server", content=client_handler.id)
            await client_handler.send_message(message)
        except Exception as error:
            logger.error(error)
            raise

    def remove_client(self, client_handler: 'ClientHandler'):
        """Remove a client from server.clients."""
        self.clients.remove(client_handler)

    async def run(self):
        """Run the server."""
        try:
            ssl_context = ssl.create_default_context(ssl.Purpose.CLIENT_AUTH)
            ssl_context.load_cert_chain(certfile=CA_FILE_PATH, keyfile=KEY_FILE_PATH)

            server = await asyncio.start_server(
                self._handle_client,
                self._server_ip,
                self._server_port,
                ssl=ssl_context,
            )

            async with server:
                logger.info(f"Server listening on: {self._server_ip}:{self._server_port}")
                print(f"Server listening on {self._server_ip}:{self._server_port}")
                await server.serve_forever()

        except Exception as error:
            logger.error(error)
            raise

    async def _handle_client(self, reader: asyncio.StreamReader, writer: asyncio.StreamWriter):
        """Handle a new client connection."""

        addr = writer.get_extra_info("peername")

        logger.info(f"New connection from: {addr[0]}:{addr[1]}")
        print(f"New connection from: {addr[0]}:{addr[1]}")

        if self._get_len_clients() < 3:
            client_handler = ClientHandler(reader, writer, self)
            client_handler.id = self._get_next_client_id()
            self._append_client(client_handler)
            await self._broadcast_user_id(client_handler)
            logger.info(f"New client added to server:{addr[0]}:{addr[1]}")
            await client_handler.run()

        else:
            await self._send_reject_message(writer)
            logger.warning(f"Client rejected from server:{addr[0]}:{addr[1]}")


class ClientHandler:
    """Class representing a client handler."""

    def __init__(self, reader: asyncio.StreamReader, writer: asyncio.StreamWriter, server: Server, id: int = None):
        """Initialize the client handler."""
        self.reader = reader
        self.writer = writer
        self.server = server
        self.id = id

    async def send_message(self, message: Message):
        """Send a message to the client."""
        try:
            self.writer.write(message.encode())
            await self.writer.drain()
        except Exception as error:
            logger.error(error)
            raise

    def is_closed(self):
        """Check if the client's connection is closed."""
        return self.writer.is_closing()

    async def run(self):
        """Run the client handler."""
        try:
            while not self.is_closed():
                raw_message = await self.reader.read(1024)
                if not raw_message:
                    break
                message = Message.create(raw_message)
                logger.debug(f"Message recived from:{self.id}")
                await self._broadcast(message)
        except Exception as error:
            logger.error(error)
            raise
        finally:
            self.disconnect()
            logger.warning(f"Client disconnected:{self.id}")

    async def _broadcast(self, message: Message):
        """Broadcast a message to server clients.
        Check for the clients who are still connected to the server."""
        for client in self.server.clients:
            if client != self and message.status == 200:
                await client.send_message(message)
                logger.debug(f"Message sended to:{client.id}")

    def disconnect(self):
        """Disconnect the client."""
        if not self.is_closed():
            self.writer.close()

        self.server.remove_client(self)
        logger.warning("Client disconnected")


if __name__ == "__main__":
    try:
        server = Server()
        asyncio.run(server.run())
    except KeyboardInterrupt:
        print("Server terminated by user.")
