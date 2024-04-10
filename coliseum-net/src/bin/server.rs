extern crate lib;

use lib::{App, CreateAccountData, CreateTransactionData, GetAccountData, Request, Response};
use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};

struct Server {
    addr: String,
    app: App,
}

impl Server {
    /// Gestiona una conexión entrante para la creación de una cuenta
    fn create_account(&mut self, request: Request, mut stream: TcpStream) {
        let data: CreateAccountData = serde_json::from_str(&request.data).unwrap();
        let account = self.app.create_account(data.username);
        let response = Response {
            origin_addr: self.addr.clone().to_string(),
            target_addr: stream.peer_addr().unwrap().to_string(),
            data: serde_json::to_string(&account).unwrap(),
            status: 200,
        };
        response.send(&mut stream);
    }

    /// Gestiona una conexión entrante para la obtención de una cuenta
    fn get_account(&mut self, request: Request, mut stream: TcpStream) {
        let data: GetAccountData = serde_json::from_str(&request.data).unwrap();
        let account = self.app.get_account(&data.account_id).unwrap();
        let response = Response {
            origin_addr: self.addr.clone().to_string(),
            target_addr: stream.peer_addr().unwrap().to_string(),
            data: serde_json::to_string(account).unwrap(),
            status: 200,
        };
        response.send(&mut stream);
    }

    /// Gestiona una conexión entrante para la creación de una transacción
    fn create_transaction(&mut self, request: Request, mut stream: TcpStream) {
        let data: CreateTransactionData = serde_json::from_str(&request.data).unwrap();
        let transaction = self
            .app
            .create_transaction(data.from_id, data.to_id, data.amount);
        let response = Response {
            origin_addr: self.addr.clone().to_string(),
            target_addr: stream.peer_addr().unwrap().to_string(),
            data: serde_json::to_string(&transaction).unwrap(),
            status: 200,
        };
        response.send(&mut stream);
    }

    /// Gestiona la conexiones entrantes según el endpoint
    fn handle_connection(&mut self, request: Request, stream: TcpStream) {
        println!("{}:{}", request.origin_addr, request.endpoint);

        // El servidor actua según el endpoint dentro de la Request
        if request.endpoint == "GetAccount" {
            self.get_account(request, stream);
        } else if request.endpoint == "CreateAccount" {
            self.create_account(request, stream);
        } else if request.endpoint == "CreateTransaction" {
            self.create_transaction(request, stream);
        } else {
            println!("Invalid endpoint");
        }
    }

    fn run(&mut self) {
        println!("Listening on: {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        for stream in listener.incoming() {
            // Procesamiento de las conexiones
            let mut stream = stream.unwrap();
            let mut buf = [0; 1024];
            let mut reader = BufReader::new(&mut stream);
            let n = reader.read(&mut buf).unwrap();
            let connection = String::from_utf8_lossy(&buf[..n]).to_string();

            // Se transforma el str a una estructura Request para procesar
            let request: Request = serde_json::from_str(&connection).unwrap();
            self.handle_connection(request, stream);
        }
    }
}

fn main() {
    let addr = "127.0.0.1:5000".to_string();
    let app = App::new(addr.clone());
    let mut server = Server { addr, app };
    server.run();
}
