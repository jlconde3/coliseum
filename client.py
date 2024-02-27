import socket
import json
import threading
import uuid


initial_message = r"""
 
 ________  ________  ___       ___  ________  _______   ___  ___  _____ ______      
|\   ____\|\   __  \|\  \     |\  \|\   ____\|\  ___ \ |\  \|\  \|\   _ \  _   \    
\ \  \___|\ \  \|\  \ \  \    \ \  \ \  \___|\ \   __/|\ \  \\\  \ \  \\\__\ \  \   
 \ \  \    \ \  \\\  \ \  \    \ \  \ \_____  \ \  \_|/_\ \  \\\  \ \  \\|__| \  \  
  \ \  \____\ \  \\\  \ \  \____\ \  \|____|\  \ \  \_|\ \ \  \\\  \ \  \    \ \  \ 
   \ \_______\ \_______\ \_______\ \__\____\_\  \ \_______\ \_______\ \__\    \ \__\
    \|_______|\|_______|\|_______|\|__|\_________\|_______|\|_______|\|__|     \|__|
                                      \|_________|                                  

Welcome to Coliseum v.0.0.0.                                                                             
                                                                                    
Considerations:

    - This is a free chat, so feel free to express yourself.
    - It guarantees complete anonymity to foster open thinking.
    - The chat is exclusively for humans, so let's be kind to each other.
    - Keep in mind that no messages are saved for privacy reasons.
    - Please stick to text only; other forms of communication are not allowed.
    - All communication is encrypted for security.
    - There is a single channel for everyone to connect.
    - In case of any issues, start your message with @issue, and the community will address it.

To start chatting, simply press Enter!

"""
import socket
import threading
import json

HOST = "127.0.0.1"
PORT = 65432
user_id = None

def print_message(data: bytes):
    message_recv: dict = json.loads(data.decode("utf-8"))
    print(f"{message_recv.get('ip')}@{message_recv.get('user')}:{message_recv.get('message')}")

def receive_messages(sock: socket.socket):
    try:
        while True:
            data = sock.recv(1024)
            if not data:
                break
            
            print_message(data)
            
    except Exception as e:
        print(f"Error receiving message: {e}")
        raise

def send_message(sock: socket.socket):
    try:
        while True:
            message = input()

            if message.lower() == 'exit':
                break

            message_dict = {
                "ip": HOST,
                "user": user_id,
                "message": message
            }

            message_json = json.dumps(message_dict).encode()
            sock.sendall(message_json)

    except Exception as e:
        print(f"Error sending message: {e}")
        raise

def initial_connection(sock: socket.socket):
    global user_id
    try:
        message_dict = {
            "ip": HOST,
            "message": "new_connection"
        }

        message_json = json.dumps(message_dict).encode()
        sock.sendall(message_json)

        # Receive the unique ID assigned by the server
        response = sock.recv(1024)
        user_id = json.loads(response.decode("utf-8")).get("content")

        print(f"You are now online! Your ID: {user_id}")

    except Exception as e:
        print(f"Error during initial connection: {e}")
        raise

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    print("Connecting to the server...")
    
    s.connect((HOST, PORT))

    initial_connection(s)

    receive_thread = threading.Thread(target=receive_messages, args=(s,))
    receive_thread.start()

    send_thread = threading.Thread(target=send_message, args=(s,))
    send_thread.start()

    # Wait for the send and receive threads to finish
    send_thread.join()
    receive_thread.join()
