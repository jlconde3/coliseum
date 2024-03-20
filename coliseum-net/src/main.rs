mod node;

use node::{handle_connection, Node};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, TcpListener},
};
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

#[tokio::main]
async fn main() {
    let port = 5000;

    let mut addr = "127.0.0.1:".to_string();
    addr.push_str(&port.to_string());

    let mut node = Node {
        addr,
        register_addr: "http://127.0.0.1:5000".to_string(),
        nodes: Arc::new(RwLock::new(HashSet::new())),
        storage: Arc::new(RwLock::new(Vec::new())),
    };

    if port != 5000 {
        node.register_node().await;
    }

    println!("Listening on: {}", &node.addr);
    let listener = TcpListener::bind(node.addr.clone()).await.unwrap();

    loop {
        let (mut socket, node_addr) = listener.accept().await.unwrap();
        println!("New connection: {}", &node_addr);
        let node_loop = node.clone();

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            match socket.read(&mut buf).await {
                Ok(n)=>{
                    let request = String::from_utf8(buf[..n].to_vec()).unwrap();
                    handle_connection(&mut socket, request, node_loop).await;
                }
                Err(err) => println!("An error ocurred:{}", err),
            }
        });
    }
}


