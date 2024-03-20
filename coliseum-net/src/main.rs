mod node;

use node::Node;

use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}};

use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};


async fn handle_connection(socket:&mut TcpStream) {
    let mut buf = [0; 1024];

    match socket.read(&mut buf).await{
        Ok(n) => {
            if n > 0 {
                let string = String::from_utf8(buf[..n].to_vec()).unwrap();
                println!("{}", &string);
            }else{
                println!("No bytes where sent by the peer");
            }
        },
        Err(err) => println!("An error ocurred:{}", err)
    }
}

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
