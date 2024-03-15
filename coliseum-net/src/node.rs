use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{Arc,RwLock},
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    id: String,
    timestamp: f64,
    content: String,
}
pub struct Node {
    pub addr: String,
    pub register_addr: String,
    pub nodes: Arc<RwLock<HashSet<String>>>,
    pub storage: Arc<RwLock<Vec<Item>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterNodeResponse {
    message: String,
    nodes:HashSet<String>
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct NodeRegistration{
    ip:String
}


impl Node {
    pub async fn register_node_in_entrypoint(&mut self) {

        let mut register_node_endpoint = self.register_addr.to_string();
        register_node_endpoint.push_str("/register/node");

        let client = Client::new();
        let res = client.post(&register_node_endpoint).send().await;

        match res {
            Ok(response) => {
                if let Ok(response_json) = response.json::<RegisterNodeResponse>().await {
                    self.nodes.write().unwrap().insert(register_node_endpoint.clone());

                    let response_nodes: HashSet<String> = response_json.nodes;

                    for node in response_nodes {
                        self.nodes.write().unwrap().insert(node);
                    }

                    println!(
                        "Node registered successfully at {}",
                        &register_node_endpoint
                    );
                } else {
                    println!("Failed to parse response JSON");
                }
            }
            Err(error) => {
                println!("An error occurred: {}", error);
            }
        }
    }

    pub fn create_item(&mut self, content: String) -> Item {
        let item = Item {
            id: Uuid::new_v4().to_string().replace("-", ""),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            content: content,
        };
        self.storage.write().unwrap().push(item.clone());
        item
    }

    pub async fn distribute_data_to_nodes(&mut self, item: &Item) {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str("application/json").unwrap(),
        );
        let client = Client::new();
        let item_json = serde_json::to_string(item).unwrap();

        let nodes_urls = self.nodes.read().unwrap().clone();
        let mut nodes_not_connection: Vec<&String> = Vec::new();

        for node_url in &nodes_urls {
            let res = client
                .post(node_url)
                .headers(headers.clone())
                .json(&item_json)
                .send()
                .await;

            match res {
                Ok(response) => {
                    if response.status() == 200 {
                        println!("Item distribute to node {}", node_url)
                    }
                }
                Err(error) => {
                    println!("An error occurred: {}", error);
                    nodes_not_connection.push(node_url);
                }
            }
        }

        for node in nodes_not_connection {
            self.nodes.write().unwrap().remove(node);
        }
    }

    pub async fn handle_item_from_client(&mut self, content: String) {
        let item = self.create_item(content);
        self.distribute_data_to_nodes(&item).await;
    }

    pub fn handle_item_from_node(&mut self, content: String) {
        let _ = self.create_item(content);
    }

    pub fn handle_new_node(&mut self, new_node_url: String) {
        if !self.nodes.read().unwrap().contains(&new_node_url) {
            println!("Node already in HashSet {}", &new_node_url);
        } else {
            self.nodes.write().unwrap().insert(new_node_url.clone());
            println!("Node added  to HashSet {}", new_node_url);
        };
    }
}

