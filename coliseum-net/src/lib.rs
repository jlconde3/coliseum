
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
pub enum Action {
    REGISTER,
    SUCCESS,
    CREATE,
    RETRIEVE,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Entity {
    NODE,
    CLIENT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub entity: Entity,
    pub action: Action,
    pub data: String,
}

impl Node {
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

    pub fn retrieve_item(&self, id: String) -> Option<Item> {
        // Recupera un item concreto a partir de su ID.

        let items = self.storage.read().unwrap();
        for item in items.iter() {
            if item.id == id {
                return Some(item.clone());
            }
        }
        None // Item not found
    }

    pub async fn distrbute_item(&self, data: String) {
        //Distribuye por la red de nodos un item recibido por un cliente de manera secuencial.

        let nodes = self.nodes.read().unwrap().clone();
        for node in nodes {
            let mut stream = TcpStream::connect(node).await.unwrap();
            make_request(&mut stream, Entity::NODE, Action::CREATE, data.clone()).await;
        }
    }

    pub async fn handle_client_connection(&mut self, socket: &mut TcpStream, request: Request) {
        // Procesamiento de la petición desde  un nodo

        match request.action {
            Action::CREATE => {
                // Crea el item
                let content = request.data.clone();
                let item = self.create_item(content);
                let data = serde_json::to_string(&item).unwrap();

                // Envia una respuesta al cliente
                make_response(socket, Entity::NODE, Action::SUCCESS, data.clone()).await;

                //Distribuye la información entre los nodos
                self.distrbute_item(data).await;
            }

            Action::RETRIEVE => {
                // Recupera un Item concreto.
                let id = request.data.clone();
                let item = self.retrieve_item(id);

                // Devuelve al cliente el item.
                make_response(
                    socket,
                    Entity::NODE,
                    Action::SUCCESS,
                    serde_json::to_string(&item).unwrap(),
                )
                .await;
            }

            Action::REGISTER => {
                //NA
            }

            Action::SUCCESS => {
                //NA
            }
        }
    }

    pub async fn handle_server_connection(&mut self, socket: &mut TcpStream, request: Request) {
        match request.action {
            Action::REGISTER => {
                // Un nuevo nodo/cliente se registra en el nodo y envía la lista de nodos.
                self.nodes.write().unwrap().insert(request.data.clone()); // Se añade el nuevo nodo.
                let nodes = self.nodes.read().unwrap().clone();
                make_response(
                    socket,
                    Entity::NODE,
                    Action::SUCCESS,
                    serde_json::to_string(&nodes).unwrap(),
                )
                .await;
            }

            Action::CREATE => {
                //Crea un item a partir de un request
                // por parte de otro nodo y se almacena en el nodo
                let content = request.data.clone();
                let _ = self.create_item(content);
            }

            Action::RETRIEVE => {
                //NA
            }

            Action::SUCCESS => {
                //NA
            }
        }
    }
}

async fn make_response(socket: &mut TcpStream, entity: Entity, action: Action, data: String) {
    // Genera una respuesta para un cliente
    let req = Request {
        entity,
        action,
        data,
    };
    let json = serde_json::to_string(&req).unwrap().into_bytes();
    socket.write_all(&json).await.unwrap();
}

async fn make_request(
    stream: &mut TcpStream,
    entity: Entity,
    action: Action,
    data: String,
) -> Request {
    // Genera una petición para un cliente
    
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