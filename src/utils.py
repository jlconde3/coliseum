import os
import json
import logging 


# Constantes empleadas en el uso del servicio
SERVER_IP = ""
SERVER_PORT = ""
SERVER_HOSTNAME = ""


def configure_logging(file_log_name):

    # Defininig the logger folder
    os.getcwd()
    os.chdir("..")
    path = os.getcwd()
    path_logs = os.path.join(path,"logs")
    if not os.path.exists(path_logs):    
        os.mkdir(path_logs)

    # Logging manager configuration
    logger = logging.getLogger("coliseum")
    logger.setLevel(logging.DEBUG)
    fh = logging.FileHandler(os.path.join(path_logs,file_log_name))
    formatter = logging.Formatter('%(asctime)s - %(name)s- %(levelname)s - %(message)s')
    fh.setFormatter(formatter)
    logger.addHandler(fh)
    return logger


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
    
    