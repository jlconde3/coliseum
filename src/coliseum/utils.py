import json
from config import logger


class Message:

    def __init__(self,sender_ip = None, sender = None, content = None, status = 200) -> None:
        self.sender_ip:str = sender_ip
        self.sender_id:str = sender
        self.status:int = status
        self.content:str = content

    def encode(self):
        try:
            return json.dumps(self.__dict__).encode("utf-8")
        except Exception as error:
            logger.error(error)
            raise

    def print(self):
        print(f"{self.sender_ip}@{self.sender_id}:{self.content}")

    @staticmethod
    def create(raw:bytes):
        """Factory pattern to create a message object from a sequence of bytes"""
        try:
            data: dict = json.loads(raw.decode("utf-8"))
            sender = data.get("sender")
            content = data.get("content")
            return Message(sender=sender, content=content)
        except Exception as error:
            logger.error(error)
            raise
    
    
    