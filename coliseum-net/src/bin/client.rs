extern crate lib;

use serde::{Deserialize, Serialize};
use lib::{Account, CreateAccountData, GetAccountData, Response};
use std::net::TcpStream;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
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

fn main() {
    /* Create a new account */
    let data = CreateAccountData {
        username: "Usuario1".to_string(),
    };

    let json = serde_json::to_string(&data).unwrap();
    let request = Request {
        origin_addr: "localhost".to_string(),
        target_addr: "127.0.0.1:5000".to_string(),
        data: json,
    };

    let response = request.send();
    print!("{}", response.data);

}
