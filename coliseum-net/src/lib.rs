use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    net::TcpStream,
    time::{SystemTime, UNIX_EPOCH},
};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    id: String,
    timestamp: f64,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Entity {
    NODE,
    CLIENT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    REGISTER,
    SUCCESS,
    CREATE,
    RETRIEVE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub entity: Entity,
    pub action: Action,
    pub data: String,
}

async fn make_response(socket: &mut TcpStream, entity: Entity, action: Action, data: String) {
    // Genera una respuesta para un cliente/nodo
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
    // Genera una petición para desde un nodo/cliente a otro nodo
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

#[derive(Debug, Clone)]
pub struct Node {
    pub addr: String,
    pub register_addr: String,
    pub nodes: HashSet<String>,
    pub storage: Vec<Item>,
}

impl Node {
    pub fn create_item(&mut self, content: String) -> Item {
        // Crea un item en el nodo a partir de la información.
        let item = Item {
            id: Uuid::new_v4().to_string().replace("-", ""),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            content: content,
        };

        self.storage.push(item.clone());
        item
    }

    pub fn retrieve_item(&self, id: String) -> Option<Item> {
        // Recupera un item almacenado en el nodo a partir de su ID.
        let items = self.storage.read().unwrap();
        for item in items.iter() {
            if item.id == id {
                return Some(item.clone());
            }
        }
        None // Item not found
    }

    pub async fn distrbute_item(&self, data: String) {
        //Distribuye por la red de nodos un item de manera secuencial.
        let nodes = self.nodes.clone();
        for node in nodes {
            let mut stream = TcpStream::connect(node).await.unwrap();
            make_request(&mut stream, Entity::NODE, Action::CREATE, data.clone()).await;
        }
    }

    /// Equivalante al view para la gestión de las acciones desde los clientes.
    pub async fn handle_client_connection(&mut self, socket: &mut TcpStream, request: Request) {
        // Procesamiento de la petición desde un cliente

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

    /// Equivalante al view para la gestión de las acciones contra/desde los nodos.
    pub async fn handle_server_connection(&mut self, socket: &mut TcpStream, request: Request) {
        match request.action {
            Action::REGISTER => {
                // Un nuevo nodo/cliente se registra en el nodo y envía la lista de nodos.
                self.nodes.insert(request.data.clone()); // Se añade el nuevo nodo.
                let nodes = self.nodes.clone();
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

pub struct Client {
    node: String,
}

impl Client {
    pub async fn send_item(&mut self, data: String) -> Item {
        // Conexión al nodo
        let mut stream = TcpStream::connect(self.node).await.unwrap();

        // Enviamos la petición al nodo y esperamos la respuesta
        let res = make_request(&mut stream, Entity::CLIENT, Action::CREATE, data).await;

        // Extraemos el item creado por el nodo
        let item: Item = serde_json::from_str(&res.data).unwrap();
        item
    }

    pub async fn retrive_item(&mut self, data: String) -> Item {
        // Conexión al nodo
        let mut stream = TcpStream::connect(self.node).await.unwrap();

        // Enviamos la petición al nodo y esperamos la respuesta
        let res = make_request(&mut stream, Entity::CLIENT, Action::RETRIEVE, data).await;

        // Extraemos el item creado por el nodo
        let item: Item = serde_json::from_str(&res.data).unwrap();
        item
    }
}
