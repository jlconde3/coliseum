
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{io::AsyncWriteExt, net::TcpStream};
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
    RETRIEVE
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum  Entity {
    NODE,
    CLIENT
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    entity: Entity,
    action: Action,
    data: String,
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
    
    pub async fn handle_client_connection(&mut self, socket: &mut TcpStream, request: Request) {
        // Procesamiento de la petición desde  un nodo

        match request.action {

            Action::CREATE => {
                // Crea un nuevo item
                let content = request.data.clone();
                let item = self.create_item(content);

                make_response(
                    socket,
                    Entity::NODE,
                    Action::SUCCESS,
                    serde_json::to_string(&item).unwrap(),
                )
                .await;
            }

            Action::RETRIEVE => {
                // Recupera un Item concreto.
                let id = request.data.clone();
                let item = self.retrieve_item(id);

                make_response(
                    socket,
                    Entity::NODE,
                    Action::SUCCESS,
                    serde_json::to_string(&item).unwrap(),
                )
                .await;
            }

            Action::REGISTER =>{
                //NA
            }

            Action::SUCCESS => {
                //NA
            }


        }
    }

    pub async fn handle_server_connection(&mut self, socket: &mut TcpStream, request: Request){

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
                //NA
            }

            Action::RETRIEVE =>{
                //NA
            }
            
            Action::SUCCESS => {
                //NA
            }
        }

        
    }
}

async fn make_response(socket: &mut TcpStream, entity:Entity, action: Action, data: String) {
    let req = Request { entity, action, data };
    let json = serde_json::to_string(&req).unwrap().into_bytes();
    socket.write_all(&json).await.unwrap();
}
