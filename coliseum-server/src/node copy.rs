use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    id: String,
    timestamp: f64,
    content: String,
}
#[derive(Debug, Clone)]
pub struct Node {
    pub addr: String,
    pub register_addr: String,
    pub nodes: Arc<RwLock<HashSet<String>>>,
    pub storage: Arc<RwLock<Vec<Item>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterNodeResponse {
    message: String,
    nodes: HashSet<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NodeRegistration {
    ip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    REGISTER,
    SUCCESS,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    action: Action,
    data: String,
}

impl Node {
    pub async fn register_node(&mut self) {
        // Conexión al nodo de registro
        let mut stream = TcpStream::connect(self.register_addr.clone())
            .await
            .unwrap();

        // Enviamos la petición al nodo de registro y esperamos la respuesta
        let res = make_request(&mut stream, Action::REGISTER, self.addr.clone()).await;

        // Extraemos los nodos incluido en el campo data de la respuesta del servidor
        let nodes: Vec<String> = serde_json::from_str(&res.data).unwrap();
        for node in nodes {
            self.nodes.write().unwrap().insert(node);
        }
    }

    pub async fn distribute_data(&mut self, item: &Item) {
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

    pub async fn handle_item_from_client(&mut self, content: String) {
        let item = self.create_item(content);
        self.distribute_data(&item).await;
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

/// Vistas
///
/// Registrarse en nodo central
/// Registrarse en un nodo de la red
/// Registrar nuevos nodos
///
/// Recivir item del cliente
/// Recivir item de un nodo
///
/// Distribuir item a todos los nodos
//
pub async fn handle_connection(socket: &mut TcpStream, request: String, node: Node) {
    // Procesamiento de la petición desde  un nodo
    let request: Request = serde_json::from_str(&request).unwrap();

    match request.action {
        Action::REGISTER => {
            // Un nuevo nodo se registra en el nodo y envía la lista de nodos.
            let nodes = &node.nodes.read().unwrap().clone();
            make_response(
                socket,
                Action::SUCCESS,
                serde_json::to_string(nodes).unwrap(),
            )
            .await;
            //algo
        }
        Action::SUCCESS => {
            //Ornitorrincos
        }
    }
}

async fn make_request(stream: &mut TcpStream, action: Action, data: String) -> Request {
    // Preparación de la petición al nodo
    let req = Request {
        action: action,
        data: data,
    };
    let json = serde_json::to_string(&req).unwrap().into_bytes();

    // Envio del mensaje al nodo
    stream.write_all(&json).await.unwrap();

    // Esperamos la respuesta del nodo
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await.unwrap();

    // Procesameos la respuesta del nodo
    let string = String::from_utf8(buf[..n].to_vec()).unwrap();
    let response: Request = serde_json::from_str(&string).unwrap();
    response
}

async fn make_response(socket: &mut TcpStream, action: Action, data: String) {
    let req = Request { action, data };
    let json = serde_json::to_string(&req).unwrap().into_bytes();
    socket.write_all(&json).await.unwrap();
}
