use serde::{Deserialize, Serialize};
use tokio::{
    net::TcpStream,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Action {
    RegisterNode,
    DistributeItem,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Request {
    action: Action,
    data: String,
}

async fn make_request(node_addr: &str, action: Action, data: String){
    let mut stream = TcpStream::connect(node_addr).await.unwrap();

    let req = Request {
        action: action,
        data: data,
    };

    let json = serde_json::to_string(&req).unwrap().into_bytes();
    stream.write_all(&json).await.unwrap();
}

pub async fn handle_connection(socket: &mut TcpStream) {
    let mut buf = [0; 1024];

    match socket.read(&mut buf).await {
        Ok(n) => {
            if n > 0 {
                let string = String::from_utf8(buf[..n].to_vec()).unwrap();
                println!("{}", &string);
            } else {
                println!("No bytes where sent by the peer");
            }
        }
        Err(err) => println!("An error ocurred:{}", err),
    }
}
