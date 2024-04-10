extern crate lib;

use chrono::Utc;
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
        let data: Result<CreateAccountData, serde_json::Error> =
            serde_json::from_str(&request.data);
        match data {
            Err(_) => {
                let response = Response {
                    origin_addr: self.addr.clone().to_string(),
                    target_addr: stream.peer_addr().unwrap().to_string(),
                    data: "Request Data is not valid".to_string(),
                    status: 401,
                };
                response.send(&mut stream);
            }

            Ok(data) => {
                let account = self.app.create_account(data.username);

                match account {
                    Ok(data) => {
                        let response = Response {
                            origin_addr: self.addr.clone().to_string(),
                            target_addr: stream.peer_addr().unwrap().to_string(),
                            data: data,
                            status: 200,
                        };
                        response.send(&mut stream);
                    }

                    // Internal server error
                    Err(error) => {
                        let response = Response {
                            origin_addr: self.addr.clone().to_string(),
                            target_addr: stream.peer_addr().unwrap().to_string(),
                            data: error,
                            status: 500,
                        };
                        response.send(&mut stream);
                    }
                }
            }
        }
    }

    /// Gestiona una conexión entrante para la obtención de una cuenta
    fn get_account(&mut self, request: Request, mut stream: TcpStream) {
        let data: Result<GetAccountData, serde_json::Error> = serde_json::from_str(&request.data);

        match data {
            Err(_) => {
                let response = Response {
                    origin_addr: self.addr.clone().to_string(),
                    target_addr: stream.peer_addr().unwrap().to_string(),
                    data: "Request Data is not valid".to_string(),
                    status: 401,
                };
                response.send(&mut stream);
            }

            Ok(data) => {
                let account = self.app.get_account(&data.account_id);
                match account {
                    Err(_) => {
                        let response = Response {
                            origin_addr: self.addr.clone().to_string(),
                            target_addr: stream.peer_addr().unwrap().to_string(),
                            data: format!("Account with ID: {} not found", data.account_id),
                            status: 402,
                        };
                        response.send(&mut stream);
                    }

                    Ok(account) => {
                        let response = Response {
                            origin_addr: self.addr.clone().to_string(),
                            target_addr: stream.peer_addr().unwrap().to_string(),
                            data: serde_json::to_string(account).unwrap(),
                            status: 200,
                        };
                        response.send(&mut stream);
                    }
                }
            }
        }
    }

    /// Gestiona una conexión entrante para la creación de una transacción
    fn create_transaction(&mut self, request: Request, mut stream: TcpStream) {
        
        let data: Result<CreateTransactionData, serde_json::Error> =
            serde_json::from_str(&request.data);

        match data {
            Err(_) => {
                let response = Response {
                    origin_addr: self.addr.clone().to_string(),
                    target_addr: stream.peer_addr().unwrap().to_string(),
                    data: "Request Data is not valid".to_string(),
                    status: 401,
                };
                response.send(&mut stream);
            }

            Ok(data) => {
                let transaction =
                    self.app
                        .create_transaction(data.from_id, data.to_id, data.amount);

                let response = Response {
                    origin_addr: self.addr.clone().to_string(),
                    target_addr: stream.peer_addr().unwrap().to_string(),
                    data: serde_json::to_string(&transaction).unwrap(),
                    status: 200,
                };
                response.send(&mut stream);
            }
        }
    }

    fn handle_incorrect_endpoint(&mut self, request: Request, mut stream: TcpStream) {
        let response = Response {
            origin_addr: self.addr.clone().to_string(),
            target_addr: stream.peer_addr().unwrap().to_string(),
            data: format!("{} endpoint not found", request.endpoint),
            status: 400,
        };

        response.send(&mut stream);
    }

    /// Gestiona la conexiones entrantes según el endpoint
    fn handle_connection(&mut self, connection: String, mut stream: TcpStream) {
        let request: Result<Request, serde_json::Error> = serde_json::from_str(&connection);

        match request {
            Err(_) => {
                println!(
                    "{} - {} - {}",
                    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                    stream.peer_addr().unwrap().to_string(),
                    "Request does not satisfied protocol"
                );

                let response = Response {
                    origin_addr: self.addr.clone().to_string(),
                    target_addr: stream.peer_addr().unwrap().to_string(),
                    data: "Request is not valid".to_string(),
                    status: 405,
                };
                response.send(&mut stream);
            }

            Ok(request) => {
                println!(
                    "{} - {} - {}",
                    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                    request.origin_addr,
                    request.endpoint
                );

                // El servidor actua según el endpoint dentro de la Request
                if request.endpoint == "GetAccount" {
                    self.get_account(request, stream);
                } else if request.endpoint == "CreateAccount" {
                    self.create_account(request, stream);
                } else if request.endpoint == "CreateTransaction" {
                    self.create_transaction(request, stream);
                } else {
                    self.handle_incorrect_endpoint(request, stream);
                }
            }
        }
    }

    fn run(&mut self) {
        println!("Listening on: {}", &self.addr);

        match TcpListener::bind(&self.addr){
            Err(_) => {
                println!("Unavaliable to bind to {}", &self.addr);
            }
            Ok(listener)=>{
                for stream in listener.incoming() {
                    // Procesamiento de las conexiones
                    let mut stream = stream.unwrap();
                    let mut buf = [0; 1024];
                    let mut reader = BufReader::new(&mut stream);
                    let n = reader.read(&mut buf).unwrap();
                    let connection = String::from_utf8_lossy(&buf[..n]).to_string();
        
                    // Se transforma el str a una estructura Request para procesar
                    self.handle_connection(connection, stream);
                }
            }
        };
    }
}

fn main() {
    let addr = "127.0.0.1:5000".to_string();
    let app = App::new(addr.clone());
    let mut server = Server { addr, app };
    server.run();
}
