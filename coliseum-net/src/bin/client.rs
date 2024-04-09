use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpStream;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    origin_addr: String,
    target_addr: String,
    data: String,
    status:u8,
}

impl Response {
    // Procesa el cuerpo/data de la respuesta y lo devuelve como una estructura concreta
    pub fn data(&self) {
        let data: Value = serde_json::from_str(&self.data).unwrap();
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
    /// Envia una petición a un servidor y devuelve su respuesta
    fn send(&self) -> Response {
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

#[derive(Serialize, Deserialize)]
pub struct GetAccountData{
    user_name:String
}

fn main(){

    let data = GetAccountData{
        user_name:"Usuario1".to_string()
    };

    let json = serde_json::to_string(&data).unwrap();
    
    let request = Request {
        endpoint : "GetAccount".to_string(),
        origin_addr: "localhost".to_string(),
        target_addr: "127.0.0.1:5000".to_string(),
        data: json
    };

    let response = request.send();
    println!("{}", response.data);
    println!("{}", response.status);
}