mod node;
mod req;

use node::Node;
use req::handle_connection;
use tokio::net::TcpListener;

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
        node.register_node_in_entrypoint().await;
    }

    let listener = TcpListener::bind(node.addr.clone()).await.unwrap();
    println!("Listening on: {}", node.addr.clone());

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("New connection: {}", addr);

        tokio::spawn(async move {
            handle_connection(&mut socket).await;
        });
    }
}
