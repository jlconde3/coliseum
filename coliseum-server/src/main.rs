mod node;

use node::Node;
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() {

    let mut node = Node {
        addr: "127.0.0.1:5000".to_string(),
        register_addr: "127.0.0.1:5000".to_string(),
        nodes: Arc::new(RwLock::new(HashSet::new())),
        storage: Arc::new(RwLock::new(Vec::new())),
    };
 
    println!("Listening on: {}", &node.addr);
    let listener = TcpListener::bind(&node.addr).await.unwrap();

    loop {
        let (mut socket, node_addr) = listener.accept().await.unwrap();
        println!("New connection: {}", node_addr);
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await.unwrap();
        let request = String::from_utf8_lossy(&buf[..n]).to_string();
        node.handle_client_connection(&mut socket, request).await;
    }
}
