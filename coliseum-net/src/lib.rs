use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/* Lógica de negocio */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub amount: f64,
    pub timestamp: f64,
    pub node: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: String,
    pub created_time: f64,
    pub last_login: f64,
    pub username: String,
    pub balance: f64,
}

impl Account {
    pub fn to_string(self) -> Result<String, String> {
        match serde_json::to_string(&self.clone()){
            Ok(data) => Ok(data),
            Err(error) => {
                println!("{}", error);
                Err("Internal Server Error".to_string())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub addr: String,
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

impl App {
    /// Create a new App/node
    pub fn new(addr: String) -> App {
        let app = App {
            addr,
            accounts: Vec::new(),
            transactions: Vec::new(),
        };
        app
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

    /// Static -> Create a new account using the username
    pub fn create_account(&mut self, username: String) -> Result<String, String> {
        let timestamp = App::create_timestamp();

        let account = Account {
            id: App::create_uuid(),
            created_time: timestamp.clone(),
            last_login: timestamp.clone(),
            username,
            balance: 0.0,
        };

        let account_str = account.clone().to_string();

        match account_str {
            Ok(data) => {
                self.accounts.push(account);
                Ok(data)
            }
            Err(error) => Err(error),
        }
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

    /// Static -> Create a new transaction but it is not check
    pub fn create_transaction(
        &mut self,
        from_id: String,
        to_id: String,
        amount: f64,
    ) -> Transaction {
        let transaction = Transaction {
            id: App::create_uuid(),
            from_id,
            to_id,
            amount,
            timestamp: App::create_timestamp(),
            node: self.addr.clone(),
        };

        transaction
    }

    /// Static -> Get all transactions stored in App
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

/* Protocolo de comuncicación */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub origin_addr: String,
    pub target_addr: String,
    pub data: String,
    pub status: u16,
}

impl Response {
    // Envía una respuesta a una petición
    pub fn send(&self, stream: &mut TcpStream) {
        let json = serde_json::to_string(&self).unwrap().into_bytes();
        stream.write_all(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub endpoint: String,
    pub origin_addr: String,
    pub target_addr: String,
    pub data: String,
}

impl Request {
    /// Envia una petición y espera su respuesta
    pub fn send(&self) -> Response {
        //Se preocesa la petición
        let json = serde_json::to_string(&self.clone()).unwrap().into_bytes();

        // Creamos la conexión y enviamos la petición
        let mut stream = TcpStream::connect(&self.target_addr).unwrap();
        stream.write_all(&json).unwrap();
        // Esperamos la respuesta del nodo
        let mut buf = [0; 1024];
        let n = stream.read(&mut buf).unwrap();

        // Procesameos la respuesta del servidor y se devuelve un objeto respuesta
        let str = String::from_utf8(buf[..n].to_vec()).unwrap();
        let response: Response = serde_json::from_str(&str).unwrap();
        response
    }
}

/* Estructuras de datos */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccountData {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountData {
    pub account_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTransactionData {
    pub from_id: String,
    pub to_id: String,
    pub amount: f64,
}
