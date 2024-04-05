use std::{
    sync::{Arc,Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use uuid::Uuid;

pub enum StatusTransaction {
    SCUCCESS,
    CANCEL,
    ERROR
}


pub struct Transaction {
    transaction_id:String,
    from_id:String,
    to_id:String,
    amount:String,
    status:StatusTransaction,
    timestamp:f64,
    node:String,

}

pub enum  Statusaccount {
    ACTIVE,
    INACTIVE
}
pub struct Account {
    account_id:String,
    account_status: Statusaccount,
    account_create_time:f64,
    account_last_login:f64,
    user_id:String,
    user_name:String,
    balance: f64,
}


pub struct Server {
    addr:String,
    accounts:Arc<Mutex<Vec<Account>>>,
    transactions:Arc<Mutex<Vec<Transaction>>>,
}


impl Server {
    pub fn new(addr:String)->Server{
        let server = Server{
            addr,
            accounts:Arc::new(Mutex::new(Vec::new())),
            transactions:Arc::new(Mutex::new(Vec::new())),
        };
        server
    }

    pub fn get_account(&mut self,  account_id:&str)-> Result<Account,()>{
        
        let accounts = self.accounts.lock().unwrap();
        for account in accounts.into_iter(){
            if &account_id == &account.account_id{
                return Ok(account);
            }else {
                continue;
            }
        };
        println!("Account with ID {} not found", account_id);
        Err(())
    }
}


impl Account {

    pub fn new(user_name:String)->Account{
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let account = Account {
            account_id: Uuid::new_v4().to_string().replace("-", ""),
            account_status: Statusaccount::ACTIVE,
            account_create_time:timestamp.clone(),
            account_last_login:timestamp.clone(),
            user_id: Uuid::new_v4().to_string().replace("-", ""),
            user_name:user_name,
            balance:0.0,
        };
        account
    }
}
