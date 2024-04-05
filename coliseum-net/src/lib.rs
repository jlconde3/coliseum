use serde::{Deserialize, Serialize};

use std::{
    collections::HashSet,
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc,Mutex},
    thread,
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
    // Genera una petición para desde un nodo/cliente a otro nodo
    let req = Request {
        entity,
        action,
        data: data.to_string(),
    };
    let json = serde_json::to_string(&req).unwrap().into_bytes();
    // Envio del mensaje al nodo
    stream.write_all(&json).unwrap();

    Ok(())
}

fn make_request(
    to_addr: &String,
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
    let mut stream = TcpStream::connect(to_addr).unwrap();
    stream.write_all(&json).unwrap();

    // Esperamos la respuesta del nodo
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).unwrap();

    // Procesameos la respuesta del nodo
    let string = String::from_utf8(buf[..n].to_vec()).unwrap();
    let response: Request = serde_json::from_str(&string).unwrap();
    Ok(response)
}

pub fn distrbute_item(addr:String, nodes:HashSet<String>, data: String) {
    //Distribuye por la red de nodos un item de manera secuencial.

    for node in nodes{
        if node == addr {
            continue;
        }
        println!("Data sending to {}", &node);
        match make_request(&node, Entity::NODE, Action::CREATE, data.clone()) {
            Ok(_) => {
                println!("Data send to {}", &node);
            }
            Err(error) => {
                println!("{}", error)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub addr: String,
    pub register_addr: String,
    pub nodes: Arc<Mutex<HashSet<String>>>,
    pub storage: Vec<Item>,
}

impl Node {
    pub fn new(addr: &String, register_addr: &String) -> Self {
        let node = Node {
            addr: addr.clone(),
            register_addr: register_addr.clone(),
            nodes: Arc::new(Mutex::new(HashSet::new())),
            storage: Vec::new(),
        };

        node.nodes.lock().unwrap().insert(addr.clone());
        node
    }

    pub fn register(&mut self) {
        match make_request(
            &self.register_addr,
            Entity::NODE,
            Action::REGISTER,
            self.addr.clone(),
        ) {
            Ok(res) => {
                let nodes: HashSet<String> = serde_json::from_str(&res.data).unwrap();
                for node in nodes {
                    if node != self.addr {
                        self.nodes.lock().unwrap().insert(node);
                    } else {
                        continue;
                    }
                }
                println!("Node register successfully");
            }
            Err(error) => println!("{}", error),
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

        // Si el item no se encuentra en el nodo, se pregunta al resto de nodos.
        // Se tiene que preguntar a un número mínimo de nodos, al menos al 51% de los almacenados.
        // Además se debe de tener en cuenta solo aquellos que permiten a hacer la consulta
        // es decir a los que estén en línea.

        //let nodes = &self.nodes.lock().unwrap().clone();

        Err(format!("Item with ID '{}' not found", id))
    }


    /// Equivalante al view para la gestión de las acciones desde los clientes.
    pub fn handle_client_connection(&mut self, stream: &mut TcpStream, request: Request) {
        // Procesamiento de la petición desde un cliente

        match request.action {
            Action::CREATE => {
                // Crea un item
                println!("Creating a new item sent by a client");
                let content = request.data.clone();

                match self.create_item(content) {
                    Ok(item) => {
                        let data = serde_json::to_string(&item).unwrap();

                        // Realizamos la siguiente en paralelo. Debido a que la operación de enviar una respuesta al cliente
                        // no es bloqueante con la función de enviar item se pueden crear un nuevo hilo para el envío del item
                        // al resto de nodos

                        // Envia una respuesta al client
                        match make_response(stream, Entity::NODE, Action::SUCCESS, data.clone()) {
                            Ok(_) => {}
                            Err(error) => {
                                println!("{}", error);
                            }
                        };

                        let nodes = self.nodes.lock().unwrap().clone();
                        let addr = self.addr.clone();

                        thread::spawn(move || distrbute_item(addr, nodes, data.clone()));
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
    pub fn handle_node_connection(&mut self, stream: &mut TcpStream, request: Request) {
        match request.action {
            Action::REGISTER => {
                // Un nuevo nodo se registra en el nodo y envía la lista de nodos.
                self.nodes.lock().unwrap().insert(request.data.clone()); // Se añade el nuevo nodo.
                let nodes = self.nodes.lock().unwrap().clone();
                let data = serde_json::to_string(&nodes).unwrap();

                match make_response(stream, Entity::NODE, Action::SUCCESS, data) {
                    Ok(_) => {
                        println!("Node {} register successfuully", request.data);
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
                let item: Item = serde_json::from_str(&request.data).unwrap();
                self.storage.push(item.clone());

                match make_response(stream, Entity::NODE, Action::SUCCESS, request.data.clone()) {
                    Ok(_) => {
                        println!("Item {} created successfuully", item.id);
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



