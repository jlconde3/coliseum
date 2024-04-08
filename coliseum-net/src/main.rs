use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

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

    /// Create a new account using the user_name
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

    /// Get an specific account query by its account ID
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

    /// Create a new transaction but it is not check
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

    /// Get all the transaction
    pub fn get_transactions(self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    /// Check if a transaction is valid
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
    let mut server = Server::new("127.0.0.1:5000".to_string());

    let account_1 = server.create_account("user_1".to_string());
    let account_2 = server.create_account("user_2".to_string());

    server.get_account(&account_1.id).unwrap().balance = 1000.0;

    let transaction_1 =
        server.create_transaction(account_1.id.clone(), account_2.id.clone(), 100.0);
    let status = server.check_transaction(transaction_1);
    println!("{}", status);

    let transaction_2 = server.create_transaction(account_1.id.clone(), account_2.id.clone(), 10.0);
    let status = server.check_transaction(transaction_2);
    println!("{}", status);

    let transaction_3 = server.create_transaction(account_2.id.clone(), account_1.id.clone(), 10.0);
    let status = server.check_transaction(transaction_3);
    println!("{}", status);

    let account = server.get_account(&account_1.id).unwrap();
    println!(
        "Account with ID: {} -> Balance: {}",
        account.id, account.balance
    );

    let account = server.get_account(&account_2.id).unwrap();
    println!(
        "Account with ID: {} -> Balance: {}",
        account.id, account.balance
    );

    for item in server.get_transactions().clone() {
        println!(
            "ID:{} from {} to {} amount:{} ",
            item.id, item.from_id, item.to_id, item.amount
        )
    }
}
