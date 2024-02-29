import os
import logging 

from dotenv import find_dotenv, load_dotenv

def configure_logging(file_log):
    # Logging manager configuration
    logger = logging.getLogger("coliseum")
    logger.setLevel(logging.DEBUG)
    fh = logging.FileHandler(file_log)
    formatter = logging.Formatter('%(asctime)s - %(name)s- %(levelname)s - %(message)s')
    fh.setFormatter(formatter)
    logger.addHandler(fh)
    return logger


path = find_dotenv()
if path:
    load_dotenv()
