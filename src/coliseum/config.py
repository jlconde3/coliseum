import logging 
from dotenv import find_dotenv, load_dotenv

def configure_logging():
    # Logging manager configuration
    logger = logging.getLogger("coliseum_client")
    logger.setLevel(logging.DEBUG)
    fh = logging.FileHandler(f".\logs\coliseum_client.log")
    formatter = logging.Formatter('%(asctime)s - %(name)s- %(levelname)s - %(message)s')
    fh.setFormatter(formatter)
    logger.addHandler(fh)
    return logger


logger = configure_logging()


path = find_dotenv()
if path:
    load_dotenv()
