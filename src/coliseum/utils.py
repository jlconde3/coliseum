import json
from dataclasses import dataclass


@dataclass
class Message:
    sender: str
    content: str

    def to_bytes(self):
        return json.dumps(self.__dict__).encode("utf-8")
    
    
def create_message(message: bytes):
    """Factory pattern to create a message object"""
    try:
        data: dict = json.loads(message.decode("utf-8"))
        sender = data.get("sender")
        content = data.get("content")
        return Message(sender=sender, content=content)
    except json.JSONDecodeError:
        # Handle JSON decoding errors
        logger.error("Error decoding JSON message")
        return None