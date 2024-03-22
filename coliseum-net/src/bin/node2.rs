

use node::{Node, Entity, Request};
use std::io::{BufReader, Read};
use std::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:5001".to_string();
    let register_addr = "127.0.0.1:5000".to_string();

    let mut node = Node::new(&addr, &register_addr);

    // Se registra el nodo en el nodo principal;
    node.register();


    println!("Listening on: {}", &node.addr);
    let listener = TcpListener::bind(&node.addr).unwrap();

    for stream in listener.incoming() {
        // Procesamiento de la entradas
        let mut stream = stream.unwrap();
        let mut buf = [0; 1024];
        let mut reader = BufReader::new(&mut stream);
        let n = reader.read(&mut buf).unwrap();
        let request_str = String::from_utf8_lossy(&buf[..n]).to_string();
        let request: Result<Request, serde_json::Error> = serde_json::from_str(&request_str);

        match request {
            Ok(request) => {
                if request.entity == Entity::NODE {
                    node.handle_node_connection(&mut stream, request);
                } else if request.entity == Entity::CLIENT {
                    node.handle_client_connection(&mut stream, request);
                }
            }
            Err(_) => {
                println!("An error ocurred parsing the request!");
            }
        }
    }
}
