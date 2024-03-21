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
    pub id: String,
    pub timestamp: f64,
    pub content: String,
}
#[derive(Debug, Clone)]
pub struct Client {
    pub addr: String,
    pub register_addr: String,
    pub clients: Arc<RwLock<HashSet<String>>>,
    pub storage: Arc<RwLock<Vec<Item>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    REGISTER,
    CREATE,
    RETRIEVE,
    SUCCESS,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Entity {
    NODE,
    CLIENT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    entity: Entity,
    action: Action,
    data: String,
}

impl Client {
    pub async fn register_client(&mut self) {
        // Conexión al nodo de registro
        let mut stream = TcpStream::connect(&self.register_addr).await.unwrap();

        // Enviamos la petición al nodo de registro y esperamos la respuesta
        let res = make_request(
            &mut stream,
            Entity::CLIENT,
            Action::REGISTER,
            self.addr.clone(),
        )
        .await;

        // Extraemos los nodos incluido en el campo data de la respuesta del servidor
        let clients: Vec<String> = serde_json::from_str(&res.data).unwrap();

        for client in clients {
            self.clients.write().unwrap().insert(client);
        }
    }

    pub async fn send_item(&mut self, data: String) -> Item {
        // Conexión al nodo
        let mut stream = TcpStream::connect(&self.register_addr).await.unwrap();

        // Enviamos la petición al nodo y esperamos la respuesta
        let res = make_request(&mut stream, Entity::CLIENT, Action::CREATE, data).await;

        // Extraemos el item creado por el nodo
        let item: Item = serde_json::from_str(&res.data).unwrap();
        item
    }

    pub async fn retrive_item(&mut self, data: String) -> Item {
        // Conexión al nodo
        let mut stream = TcpStream::connect(&self.register_addr).await.unwrap();

        // Enviamos la petición al nodo y esperamos la respuesta
        let res = make_request(&mut stream, Entity::CLIENT, Action::RETRIEVE, data).await;

        // Extraemos el item creado por el nodo
        let item: Item = serde_json::from_str(&res.data).unwrap();
        item
    }
}

async fn make_request(
    stream: &mut TcpStream,
    entity: Entity,
    action: Action,
    data: String,
) -> Request {
    // Preparación de la petición al nodo
    let req = Request {
        entity,
        action,
        data,
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

#[tokio::main]
async fn main() {
    let mut client = Client {
        addr: "127.0.0.1:5002".to_string(),
        register_addr: "127.0.0.1:5000".to_string(),
        clients: Arc::new(RwLock::new(HashSet::new())),
        storage: Arc::new(RwLock::new(Vec::new())),
    };

    println!("-----------REGISTRANDO AL CLIENTE EN EL NODO------------");

    client.register_client().await;
    let clients = client.clients.read().unwrap().clone();

    for client in clients {
        println!("{}", client)
    }

    println!("-----------CREANDO UN ITEM------------");

    let item = client
        .send_item("ESTO ES UN ITEM ENVIADO".to_string())
        .await;

    println!("{}", &item.id);
    println!("{}", item.timestamp);
    println!("{}", item.content);

    println!("-----------RECUPERANDO UN ITEM------------");

    let item = client.retrive_item(item.id).await;
    println!("{}", &item.id);
    println!("{}", item.timestamp);
    println!("{}", item.content);
}
