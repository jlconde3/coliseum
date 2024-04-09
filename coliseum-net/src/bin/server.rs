use serde::{Deserialize, Serialize};
use std::io::{Read, Write, BufReader};
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
 
/* Lógica de la petición de un cliente a un servidor */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub endpoint: String,
    pub origin_addr: String,
    pub target_addr: String,
    pub data: String,
}

/* Lógica para el envio de respuesta del servidor a un cliente */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    origin_addr: String,
    target_addr: String,
    data: String,
    status:u8,
}

impl Response {
    // Envía una
    pub fn send(&self, stream: &mut TcpStream) {
        let json = serde_json::to_string(&self).unwrap().into_bytes();
        stream.write_all(&json).unwrap();
    }
}

/* Lógica de los ENDPOINTS */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccountData {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountData {
    account_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTransactionData {
    from_id: String,
    to_id: String,
    amount: f64,
}

/* Lógica de negocio */
#[derive(Debug, Clone)]
pub struct Transaction {
    id: String,
    from_id: String,
    to_id: String,
    amount: f64,
    timestamp: f64,
    node: String,
}

#[derive(Debug, Clone)]
pub struct Account {
    id: String,
    created_time: f64,
    last_login: f64,
    user_name: String,
    balance: f64,
}

pub struct Server {
    addr: String,
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
}

impl Server {
    /// Create a new server/node
    pub fn new(addr: String) -> Server {
        let server = Server {
            addr,
            accounts: Vec::new(),
            transactions: Vec::new(),
        };
        server
    }

    /// Create a UUID
    pub fn create_uuid() -> String {
        let uuid = Uuid::new_v4().to_string().replace("-", "");
        uuid
    }

    /// Create the actual timestamp since UNIX EPOCH
    pub fn create_timestamp() -> f64 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        timestamp
    }

    /// Static -> Create a new account using the user_name
    pub fn create_account(&mut self, user_name: String) -> Account {
        let timestamp = Server::create_timestamp();
        let account = Account {
            id: Server::create_uuid(),
            created_time: timestamp.clone(),
            last_login: timestamp.clone(),
            user_name,
            balance: 0.0,
        };
        self.accounts.push(account.clone());
        account
    }

    /// Dyanamic -> Get an specific account query by its account ID
    pub fn get_account(&mut self, account_id: &str) -> Result<&mut Account, ()> {
        for account in self.accounts.iter_mut() {
            if account_id == account.id {
                return Ok(account);
            } else {
                continue;
            }
        }
        println!("Account with ID {} not found", account_id);
        Err(())
    }

    ///Static -> Create a new transaction but it is not check
    pub fn create_transaction(
        &mut self,
        from_id: String,
        to_id: String,
        amount: f64,
    ) -> Transaction {
        let transaction = Transaction {
            id: Server::create_uuid(),
            from_id,
            to_id,
            amount,
            timestamp: Server::create_timestamp(),
            node: self.addr.clone(),
        };

        transaction
    }

    /// Static -> Get all transactions stored in Server
    pub fn get_all_transactions(self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    // Static -> Get an specific transaction query by ID
    pub fn get_transaction(self, transaction_id: String) -> Result<Transaction, ()> {
        for transaction in self.transactions.clone() {
            if transaction_id == transaction.id {
                return Ok(transaction);
            } else {
                continue;
            }
        }
        return Err(());
    }

    // Static -> Get an specific transaction query by ID
    pub fn get_transaction_by_account(self, account_id: String) -> Vec<Transaction> {
        let mut transactions = Vec::new();
        for transaction in self.transactions.clone() {
            if account_id == transaction.from_id {
                transactions.push(transaction)
            } else {
                continue;
            }
        }
        transactions
    }

    /// Static -> Check if a transaction is valid
    pub fn check_transaction(&mut self, transaction: Transaction) -> bool {
        let account_from = self.get_account(&transaction.from_id);

        match account_from {
            Ok(account) => {
                if account.balance > transaction.amount {
                    println!("La cuenta  origen tiene fondos suficientes");
                    account.balance = account.balance - transaction.amount.clone();
                } else {
                    println!(
                        "La cuenta origen no tiene fondos suficientes {}",
                        account.balance
                    );
                    return false;
                }
            }
            Err(()) => {
                return false;
            }
        };

        let account_to = self.get_account(&transaction.to_id);

        match account_to {
            Ok(account) => {
                println!("La cuenta de destino existe");
                account.balance = account.balance + transaction.amount.clone();
            }
            Err(()) => {
                return false;
            }
        }

        self.transactions.push(transaction.clone());
        true
    }
}

fn main() {

    let addr = "127.0.0.1:5000".to_string();
    let server = Server::new(addr);

    println!("Listening on: {}", &server.addr);
    let listener = TcpListener::bind(&server.addr).unwrap();

    for stream in listener.incoming() {

        // Procesamiento de las conexiones
        let mut stream = stream.unwrap();
        let mut buf = [0; 1024];
        let mut reader = BufReader::new(&mut stream);
        let n = reader.read(&mut buf).unwrap();
        let connection = String::from_utf8_lossy(&buf[..n]).to_string();
        
        // Se transforma el str a una estructura Request para procesar
        let request: Request = serde_json::from_str(&connection).unwrap();

        // El servidor actua según el endpoint dentro de la Request
        if request.endpoint == "GetAccount" {
            println!("Handeling GetAccount endpoint");
            println!("{}", request.origin_addr);
            println!("{}", request.data);

            let response = Response{
                origin_addr:server.addr.clone().to_string(),
                target_addr:stream.peer_addr().unwrap().to_string(),
                data:request.data,
                status:200
            };

            response.send(&mut stream);

        } else if request.endpoint == "CreateAccount" {
            println!("Handeling  CreateAccount endpoint");
            println!("{}", request.origin_addr);
            println!("{}", request.data);
        } else if request.endpoint == "CreateTransaction" {
            println!("Handeling CreateTransaction endpoint");
            println!("{}", request.data);
        } else {
            println!("Invalid endpoint");
        }
    }
}
