import json


class Message:

    def __init__(self, sender_id = None, status = 200, content = None) -> None:
        self.sender_id:str = sender_id
        self.status:int = status
        self.content:str = content

    def encode(self):
        try:
            return json.dumps(self.__dict__).encode("utf-8")
        except Exception as error:
            raise

    def print(self):
        print(f"{self.sender_id}:{self.content}")

    @staticmethod
    def create(raw:bytes):
        """Factory pattern to create a message object from a sequence of bytes"""
        try:
            data: dict = json.loads(raw.decode("utf-8"))
            sender_id = data.get("sender_id")
            status = data.get("status")
            content = data.get("content")
            return Message(sender_id=sender_id, status=status, content=content)
        except Exception as error:
            raise
    
    
    