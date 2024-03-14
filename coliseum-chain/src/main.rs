use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

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
}

impl Blockchain {
    /// Initializes a new instance of the blockchain.
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            transactions: Mutex::new(Vec::new()),
            blocks: Mutex::new(Vec::new()),
        };

        blockchain.create_block("1", 1);
        blockchain
    }

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

    /// Adds a new transaction to the list of pending transactions.
    pub fn add_transaction(&mut self, transaction: Transaction) -> Transaction {
        self.transactions.lock().unwrap().push(transaction.clone());
        transaction
    }

    /// Returns the last block in the blockchain.
    pub fn get_last_block(&self) -> Block {
        let last_block = self.blocks.lock().unwrap().last().unwrap().clone();
        last_block
    }

    pub fn get_block_hash(block: &Block) -> String {
        let serialized = serde_json::to_string(block).unwrap();
        let hash = sha256::digest(serialized);
        hash
    }

    /// Checks if a given proof is valid based on the previous proof.
    pub fn check_proof(previous_proof: &usize, proof: &usize) -> bool {
        let mut proofs = previous_proof.to_owned().to_string();
        let proof_str = proof.to_string();
        proofs.push_str(&proof_str);
        let hashed_proofs = sha256::digest(&proofs);

        for i in hashed_proofs.chars().take(6) {
            if i != '0' {
                return false;
            }
        }
        true
    }

    /// Finds a valid proof of work for the blockchain.
    pub fn obtain_proof(&self) -> usize {
        let previous_proof = self.get_last_block().proof;
        let mut proof: usize = 1;
        while !Blockchain::check_proof(&previous_proof, &proof) {
            proof += 1;
        }
        proof
    }
}

#[macro_use]
extern crate rocket;

#[get("/mine")]
fn mine(blockchain_state: &rocket::State<Mutex<Blockchain>>) -> String {
    let mut blockchain = blockchain_state.inner().lock().unwrap();
    let proof = blockchain.obtain_proof();
    let reward = Transaction {
        sender: "0".to_string(),
        receiver: "0".to_string(),
        amount: 1,
    };
    blockchain.add_transaction(reward);
    let last_block = blockchain.get_last_block();
    let previous_hash = Blockchain::get_block_hash(&last_block);
    let new_block = blockchain.create_block(&previous_hash, proof);
    serde_json::to_string(&new_block).unwrap()
}

#[get("/chain")]
fn chain(blockchain_state: &rocket::State<Mutex<Blockchain>>) -> String {
    let blockchain = blockchain_state.inner().lock().unwrap();
    serde_json::to_string(&blockchain.blocks).unwrap()
}

#[post("/transaction", data = "<transaction>")]
async fn transaction(
    transaction: Json<Transaction>,
    blockchain_state: &rocket::State<Mutex<Blockchain>>,
) -> String {
    let mut blockchain = blockchain_state.inner().lock().unwrap();
    let transaction = blockchain.add_transaction(transaction.into_inner());
    serde_json::to_string(&transaction).unwrap()
}

#[launch]
fn rocket() -> _ {
    let port = 5000;
    let blockchain = Mutex::new(Blockchain::new());

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", port)))
        .manage(blockchain)
        .mount("/", routes![mine, chain, transaction])
}
