use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

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
        let mut stream = TcpStream::connect(&self.register_addr)
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
