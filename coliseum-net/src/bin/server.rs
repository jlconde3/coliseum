use serde::{Deserialize, Serialize};
use std::io::{Read, Write, BufReader};
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
 
/* Lógica de la petición de un cliente a un servidor */

#[derive(Serialize, Deserialize, Debug, Clone)]
 struct Request {
     endpoint: String,
     origin_addr: String,
     target_addr: String,
     data: String,
}

/* Lógica para el envio de respuesta del servidor a un cliente */
#[derive(Serialize, Deserialize, Debug, Clone)]
 struct Response {
    origin_addr: String,
    target_addr: String,
    data: String,
    status:u8,
}

impl Response {
    // Envía una
     fn send(&self, stream: &mut TcpStream) {
        let json = serde_json::to_string(&self).unwrap().into_bytes();
        stream.write_all(&json).unwrap();
    }
}

/* Lógica de los ENDPOINTS */

#[derive(Serialize, Deserialize, Debug, Clone)]
 struct CreateAccountData {
    user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
 struct GetAccountData {
    account_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
 struct CreateTransactionData {
    from_id: String,
    to_id: String,
    amount: f64,
}

/* Lógica de negocio */

#[derive(Serialize, Deserialize, Debug, Clone)]
 struct Transaction {
    id: String,
    from_id: String,
    to_id: String,
    amount: f64,
    timestamp: f64,
    node: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
 struct Account {
    id: String,
    created_time: f64,
    last_login: f64,
    user_name: String,
    balance: f64,
}

#[derive(Debug, Clone)]
 struct Server {
    addr: String,
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
}

impl Server {
    /// Create a new server/node
     fn new(addr: String) -> Server {
        let server = Server {
            addr,
            accounts: Vec::new(),
            transactions: Vec::new(),
        };
        server
    }

    /// Create a UUID
     fn create_uuid() -> String {
        let uuid = Uuid::new_v4().to_string().replace("-", "");
        uuid
    }

    /// Create the actual timestamp since UNIX EPOCH
     fn create_timestamp() -> f64 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        timestamp
    }

    /// Static -> Create a new account using the user_name
     fn create_account(&mut self, user_name: String) -> Account {
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
     fn get_account(&mut self, account_id: &str) -> Result<&mut Account, ()> {
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
     fn create_transaction(
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
     fn get_all_transactions(self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    // Static -> Get an specific transaction query by ID
     fn get_transaction(self, transaction_id: String) -> Result<Transaction, ()> {
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
     fn get_transaction_by_account(self, account_id: String) -> Vec<Transaction> {
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
     fn check_transaction(&mut self, transaction: Transaction) -> bool {
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

struct App {
    server: Server,
    stream:TcpStream, 
    request: Request,
}

impl App {

    /// Gestion para el clinete la creación de una cuenta
    fn create_account(&mut self){

        println!("Handeling CreateAccount endpoint");
        println!("{}", self.request.origin_addr);
    
        let data:CreateAccountData = serde_json::from_str(&self.request.data).unwrap();
        let account = self.server.create_account( data.user_name);

        let response = Response {
            origin_addr:self.server.addr.clone().to_string(),
            target_addr:self.stream.peer_addr().unwrap().to_string(),
            data:serde_json::to_string(&account).unwrap(),
            status:200
        };
    
        response.send(&mut self.stream);
    }
    
    /// Gestiona para el cliente la obtención de una cuenta
    fn get_account(&mut self){

        println!("Handeling  GetAccount endpoint");
        println!("{}", self.request.origin_addr);

        let data:GetAccountData = serde_json::from_str(&self.request.data).unwrap();
        println!("{}", data.account_id);
    
        let response = Response {
            origin_addr:self.server.addr.clone().to_string(),
            target_addr:self.stream.peer_addr().unwrap().to_string(),
            data:self.request.data.clone(),
            status:200
        };
        response.send(&mut self.stream);
    }

    fn create_transaction(&mut self){
        println!("Handeling  CreateTransaction endpoint");
        println!("{}", self.request.origin_addr);

        let data:CreateTransactionData = serde_json::from_str(&self.request.data).unwrap();

        let transaction = self.server.create_transaction(data.from_id, data.to_id, data.amount);
    
        let response = Response {
            origin_addr:self.server.addr.clone().to_string(),
            target_addr:self.stream.peer_addr().unwrap().to_string(),
            data:serde_json::to_string(&transaction).unwrap(),
            status:200
        };
        response.send(&mut self.stream);
    }

    fn handle_connection(&mut self){

        // El servidor actua según el endpoint dentro de la Request
        if self.request.endpoint == "GetAccount" {
            self.get_account();
        } else if self.request.endpoint == "CreateAccount" {
            self.create_account();
        } else if self.request.endpoint == "CreateTransaction" {
            self.create_transaction();
        } else {
            println!("Invalid endpoint");
        }
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

        let mut app = App {
            stream,
            server:server.clone(),
            request:request.clone(),
        };

        app.handle_connection();
    }
}
