mod node;
mod req;

use std::{
    collections::HashSet,
    sync::{Arc,RwLock}
};

use tokio::{self, io::BufStream, net::TcpListener};
use node::Node;



#[tokio::main]
async fn main(){

    let port = 5000;    

    let mut addr = "127.0.0.1:".to_string();
    addr.push_str(&port.to_string());

    let mut node = Node{
        addr,
        register_addr: "http://127.0.0.1:5000".to_string(),
        nodes: Arc::new(RwLock::new(HashSet::new())),
        storage: Arc::new(RwLock::new(Vec::new())),
    };

    if port != 5000 {
        node.register_node_in_entrypoint().await;
    }

    let listener = TcpListener::bind(node.addr.clone()).await.unwrap();
    println!("listening on: {}", listener.local_addr().unwrap());

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        let mut stream = BufStream::new(stream);

        // do not block the main thread, spawn a new task
        tokio::spawn(async move {
            println!("New connection {}", addr);

            match req::parse_request(&mut stream).await {
                Ok(_) => println!("Incoming request"),
                Err(error) => {
                    println!("failed to parse request: {}", error);
                }
            }
        });
    }

}

