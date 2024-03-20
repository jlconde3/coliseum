mod node;

use node::Node;
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};


#[tokio::main]
async fn main() {

    let mut node = Node {
        addr: "127.0.0.1:5002".to_string(),
        register_addr: "127.0.0.1:5000".to_string(),
        nodes: Arc::new(RwLock::new(HashSet::new())),
        storage: Arc::new(RwLock::new(Vec::new())),
    };

    node.register_node().await;

    let nodes = node.nodes.read().unwrap().clone();

    for node in nodes{
        println!("{}", node)
    }

}
