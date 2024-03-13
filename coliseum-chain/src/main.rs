
use rocket::serde::json::Json;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    ip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: f64,
    pub transactions: Vec<Transaction>,
    pub proof: usize,
    pub previous_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub blocks: Mutex<Vec<Block>>,
    pub transactions: Mutex<Vec<Transaction>>,
    pub nodes: Mutex<Vec<String>>,
}

impl Blockchain {
    ///Inicialización de la Blockchain.
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            transactions: Mutex::new(Vec::new()),
            blocks: Mutex::new(Vec::new()),
            nodes: Mutex::new(Vec::new()),
        };

        // Creación del bloque primigenio
        blockchain.create_block("1", 1);
        blockchain
    }

    //Crea un nuevo bloque con las transacciones y lo añade a la lista de bloques de la blockchain.
    pub fn create_block(&mut self, previous_hash: &str, proof: usize) -> Block {
        let block = Block {
            index: self.blocks.lock().unwrap().len() + 1,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            transactions: self.transactions.lock().unwrap().clone(),
            proof,
            previous_hash: previous_hash.to_string(),
        };
        self.blocks.lock().unwrap().push(block.clone());
        self.transactions.lock().unwrap().clear();
        block
    }

    /// Añade una transacción a la lista de transacciones de un nodo.
    pub async fn add_transaction(&mut self, transaction:Transaction) -> Transaction {
        self.transactions.lock().unwrap().push(transaction.clone());
        transaction
    }

    /// Return the last block in the blockchain.
    pub fn get_last_block(&self) -> Block {
        let last_block = self.blocks.lock().unwrap().last().unwrap().clone();
        last_block
    }

    pub fn get_block_hash(block: &Block) -> String {
        //Genera el hash de un bloque completo.

        let serialized = serde_json::to_string(block).unwrap();
        let hash = sha256::digest(serialized);
        hash
    }

    /// Comprueba que para cierto valor la "Proof-Of-Work" es correcta.
    pub fn check_proof(previous_proof: &usize, proof: &usize) -> bool {
        // Añadimos a la memoria el &usize y se convierta a String
        let mut proofs = previous_proof.to_owned().to_string();
        let proof_str = proof.to_string();
        // Concatenación de los "Proofs"
        proofs.push_str(&proof_str);
        // Generación del Hash
        let hashed_proofs = sha256::digest(&proofs);
        // Comprobamos que los 4 primeros dígitos del Hash tengan 0. Si es así devulve Ok si no Error.
        for i in hashed_proofs.chars().take(6) {
            if i != '0' {
                return false;
            }
        }
        true
    }

    /// Obtiene la solución al "Proof-Of-Work"
    pub async fn obtain_proof(&self) -> usize {
        let previous_proof = self.get_last_block().proof;
        let mut proof: usize = 1;
        while !Blockchain::check_proof(&previous_proof, &proof) {
            proof += 1;
        }
        proof
    }

    ///Registra un nuevo nodo
    pub fn register_node(&mut self, node: Node) {
        let nodes = self.nodes.lock().unwrap();
        let new_node_ip = node.ip.clone();

        // Comprueba que el nodo no esté ya incluido
        if !nodes.contains(&new_node_ip) {
            self.nodes.lock().unwrap().push(new_node_ip);
        }
    }

    /// Comprueba que para una cadena de bloques, cada unas de
    /// las pruebas de trabajo sean correctas y que los hashes de los bloques lo son.
    /// Esta operación garantiza que la cadena recibida de un nodo sea correcta.
    fn valid_chain(&self, chain: &Blockchain) -> bool{

        let blocks =  chain.blocks.lock().unwrap().clone();

        let mut current_index = 2;

        while current_index < blocks.len() {

            // Obtenmos el bloque anterior
            let previous_block = blocks.get(current_index-1).unwrap();
            let previous_hash = Blockchain::get_block_hash(&previous_block);

            let current_block = blocks.get(current_index).unwrap();
            
            // Comprobamos el que hash de un bloque coincide con el esperado en el siguiente bloque.
            if previous_hash !=  current_block.previous_hash{
                return false;
            }

            if !Blockchain::check_proof(&previous_block.proof, &current_block.proof){
                return  false;
            }

            current_index+=1;
        }

        true
    }

    /// Contacta con otros nodos y comprueba la validez de la cadena, si es correcta, la asume como propia.
    async fn resolve_conflicts(&mut self){

        let mut len_chain = self.blocks.lock().unwrap().len();
 
        for node in self.nodes.lock().unwrap().iter(){

            node.to_string().push_str("/chain");

            match reqwest::get(node).await {
                Ok(response) => {

                    match response.json::<Blockchain>().await {
                        Ok(chain) => {
                            if self.valid_chain(&chain) && chain.blocks.lock().unwrap().len() > len_chain{
                                len_chain = chain.blocks.lock().unwrap().len();
                                self.blocks.lock().unwrap().clear();
                                self.blocks = chain.blocks;
                
                            }else{
                                continue;
                            }
                        }
                        Err(e) => {
                            println!("An error ocrrued parsing the chain from a node: {:?}",e);
                        }
                    }
                }
                Err(e) => {
                    println!("An error ocrrued contacting with the other node{:?}", e);
                }
            }
            
        }
    }
}


pub fn check_proof(previous_proof: &usize, proof: &usize) -> bool {
    // Añadimos a la memoria el &usize y se convierta a String
    let mut proofs = previous_proof.clone().to_string();
    let proof_str = proof.to_string();
    proofs.push_str(&proof_str);
    // Generación del Hash
    let hashed_proofs = sha256::digest(&proofs);
    // Comprobamos que los 4 primeros dígitos del Hash tengan 0. Si es así devulve Ok si no Error.
    for i in hashed_proofs.chars().take(5) {
        if i != '0' {
            return false;
        }
    }
    true
}

pub fn obtain_proof(&previous_proof: &usize) -> usize {
    let mut proof: usize = 1;
    while !check_proof(&previous_proof, &proof) {
        proof += 1;
    }
    proof
}

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "RustyCoin"
}


#[get("/mine")]
fn mine(blockchain_state: &rocket::State<Mutex<Blockchain>>) -> String {
    let mut blockchain = blockchain_state.inner().lock().unwrap();
    // Obtenemos la prueba
    let proof = blockchain.obtain_proof();
    // La última transacción es una recompensa para el minador.
    blockchain.add_transaction(&"0", &"0", 1);
    let last_block = blockchain.get_last_block();
    //Generación del hash del último bloque minado.
    let previous_hash = Blockchain::get_block_hash(&last_block);
    let new_block = blockchain.create_block(&previous_hash, proof);
    // Se devuelve el último bloque.
    serde_json::to_string(&new_block).unwrap()
}


/// Endpoint empleados en la conexión incial de un cliente contra el servidor

#[get("/nodes")]
fn nodes(blockchain_state: &rocket::State<Mutex<Blockchain>>) -> String {
    // Devuelve al cliente una lista con los nodos en la red de RustyCoin.
    // Esta información es necesaria para hacer la conexión incial del nuevo nodo.
    let blockchain = blockchain_state.inner().lock().unwrap();
    // Devolvemos una vector con los nodos registrados en la red
    serde_json::to_string(&blockchain.nodes).unwrap()
}


#[post("/nodes/register", data = "<node>")]
fn nodes_register(node: Json<Node>, blockchain_state: &rocket::State<Mutex<Blockchain>>) -> String {
    //Registra un nuevo nodo (en el servidor) en la lista de nodos y le devuelve la lista de nodos registrados.
    let mut blockchain = blockchain_state.inner().lock().unwrap();
    blockchain.register_node(node);
    //Se devuelve al cliente/nodo la blockchain usada en ese nodo.
    serde_json::to_string(&blockchain.nodes).unwrap()
}

#[get("/chain")]
fn chain(blockchain_state: &rocket::State<Mutex<Blockchain>>) -> String {
    let blockchain = blockchain_state.inner().lock().unwrap();
    serde_json::to_string(&blockchain.blocks).unwrap()
}


/// Endpoint empleado por nodo para transmitir una nueva transacción hacia el resto de nodos.
/// 
#[post("/transaction", data = "<transaction>")]
async fn transaction(
    transaction: Json<Transaction>,
    blockchain_state: &rocket::State<Mutex<Blockchain>>,
) -> String {

    let sender = transaction.sender.clone();
    let receiver = transaction.receiver.clone();
    let amount = transaction.amount.clone();
    
    let mut blockchain = blockchain_state.inner().lock().unwrap();
    let nodes = blockchain.nodes.clone().iter();

    let transaction = blockchain.add_transaction(&sender, &receiver, amount);

    for node in nodes{

    }

    serde_json::to_string(&transaction).unwrap()
}


/// Endpoint que emplea un cliente(nodo) para indicar al servidor (nodo actual) que ha encontrado una solución.

#[post("/block/new", data = "<block>")]
fn new_block(block: Json<Block>, blockchain_state: &rocket::State<Mutex<Blockchain>>)-> &'static str {

    let blockchain = blockchain_state.inner().lock().unwrap();
    let last_block = blockchain.get_last_block();

    if block.previous_hash != last_block.previous_hash{
        return "No correct";
    }

    if !Blockchain::check_proof(&last_block.proof, &block.proof){
        return "No correct";
    }

    blockchain.blocks.lock().unwrap().push(block.into_inner());

    "Correct"
}


#[launch]
fn rocket() -> _ {
    let port = 5000;
    let blockchain = Mutex::new(Blockchain::new());

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", port)))
        .manage(blockchain)
        .mount(
            "/",
            routes![index, mine, chain, transaction, nodes, nodes_register, new_block],
        )
}
