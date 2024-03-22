use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    io::{Read, Write},
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

fn make_response(
    stream: &mut TcpStream,
    entity: Entity,
    action: Action,
    data: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Genera una respuesta para un cliente/nodo
    let req = Request {
        entity,
        action,
        data: data.to_string(),
    };
    let json = serde_json::to_string(&req).unwrap().into_bytes();
    stream.write_all(&json).unwrap();
    Ok(())
}

fn make_request(
    stream: &mut TcpStream,
    entity: Entity,
    action: Action,
    data: String,
) -> Result<Request, Box<dyn std::error::Error>> {
    // Genera una petición para desde un nodo/cliente a otro nodo
    let req = Request {
        entity,
        action,
        data: data.to_string(),
    };
    let json = serde_json::to_string(&req).unwrap().into_bytes();

    // Envio del mensaje al nodo
    stream.write_all(&json).unwrap();

    // Esperamos la respuesta del nodo
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).unwrap();

    // Procesameos la respuesta del nodo
    let string = String::from_utf8(buf[..n].to_vec()).unwrap();
    let response: Request = serde_json::from_str(&string).unwrap();
    Ok(response)
}

#[derive(Debug, Clone)]
pub struct Node {
    pub addr: String,
    pub register_addr: String,
    pub nodes: HashSet<String>,
    pub storage: Vec<Item>,
}

impl Node {
    pub fn new(addr: &String, register_addr: &String) -> Self {
        let node = Node {
            addr: addr.clone(),
            register_addr: register_addr.clone(),
            nodes: HashSet::new(),
            storage: Vec::new(),
        };
        node
    }

    pub fn register(&mut self){

        let mut stream = TcpStream::connect(&self.register_addr).unwrap();
        
        match make_request(&mut stream, Entity::NODE, Action::REGISTER, self.addr.clone()){
            Ok(res) =>{
                let nodes:HashSet<String> = serde_json::from_str(&res.data).unwrap();
                for node in nodes{
                    self.nodes.insert(node);
                }
            }
            Err(error)=>println!("{}", error)
        }

    }

    pub fn create_item(&mut self, content: String) -> Result<Item, Box<dyn std::error::Error>> {
        // Crea un item en el nodo a partir de la información.
        let item = Item {
            id: Uuid::new_v4().to_string().replace("-", ""),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            content,
        };
        self.storage.push(item.clone());
        Ok(item)
    }

    pub fn retrieve_item(&self, id: &str) -> Result<Item, String> {
        // Retrieve an item stored in the node based on its ID.

        for item in &self.storage {
            if item.id == id {
                return Ok(item.clone());
            }
        }
        Err(format!("Item with ID '{}' not found", id))
    }

    pub fn distrbute_item(&self, data: String) {
        //Distribuye por la red de nodos un item de manera secuencial.
        for node in &self.nodes {
            let mut stream = TcpStream::connect(node).unwrap();
            match make_request(&mut stream, Entity::NODE, Action::CREATE, data.clone()) {
                Ok(_) => {
                    // Nada
                }
                Err(error) => {
                    println!("{}", error)
                }
            }
        }
    }

    /// Equivalante al view para la gestión de las acciones desde los clientes.
    pub fn handle_client_connection(&mut self, stream: &mut TcpStream, request: Request) {
        // Procesamiento de la petición desde un cliente

        match request.action {
            Action::CREATE => {
                // Crea el item
                println!("Creating a new item sent by a client");
                let content = request.data.clone();
                match self.create_item(content) {
                    Ok(item) => {
                        let data = serde_json::to_string(&item).unwrap();
                        // Envia una respuesta al cliente
                        match make_response(stream, Entity::NODE, Action::SUCCESS, data.clone()) {
                            Ok(_) => {
                                //Distribuye la información entre los nodos
                                self.distrbute_item(data);
                            }
                            Err(error) => {
                                println!("{}", error);
                            }
                        };
                    }
                    Err(error) => {
                        println!("{}", error);
                    }
                }
            }

            Action::RETRIEVE => {
                // Recupera un Item concreto.

                let id = request.data;
                println!("Retrieving a item requested by a client => {}", &id);

                match self.retrieve_item(&id) {
                    Ok(item) => {
                        let data = serde_json::to_string(&item).unwrap();
                        match make_response(stream, Entity::NODE, Action::SUCCESS, data.clone()) {
                            Ok(_) => {
                                //
                            }
                            Err(error) => {
                                println!("{}", error);
                            }
                        };
                    }
                    Err(error) => {
                        println!("{}", error);
                    }
                }
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
    pub fn handle_server_connection(&mut self, stream: &mut TcpStream, request: Request) {
        match request.action {
            Action::REGISTER => {

                println!("Registering a new node");

                // Un nuevo nodo se registra en el nodo y envía la lista de nodos.
                self.nodes.insert(request.data.clone()); // Se añade el nuevo nodo.
                let nodes = self.nodes.clone();
                let data = serde_json::to_string(&nodes).unwrap();

                match make_response(stream, Entity::NODE, Action::SUCCESS, data) {
                    Ok(_) => {
                        //Nada
                    }
                    Err(error) => {
                        println!("{}", error);
                    }
                }
            }

            Action::CREATE => {
                // Crea el item
                println!("Creating an item from another node");
                
                let content = request.data.clone();
                match self.create_item(content) {
                    Ok(item) => {
                        let data = serde_json::to_string(&item).unwrap();
                        // Envia una respuesta al cliente
                        match make_response(stream, Entity::NODE, Action::SUCCESS, data.clone()) {
                            Ok(_) => {}
                            Err(error) => {
                                println!("{}", error);
                            }
                        };
                    }
                    Err(error) => {
                        println!("{}", error);
                    }
                }
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
