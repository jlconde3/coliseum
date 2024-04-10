extern crate lib;
use lib::{CreateAccountData, Request};



fn main() {
    /* Create a new account */
    let data = CreateAccountData {
        username: "Usuario1".to_string(),
    };

    let json = serde_json::to_string(&data).unwrap();
    let request = Request {
        endpoint:"CreateAccount".to_string(),
        origin_addr: "localhost".to_string(),
        target_addr: "127.0.0.1:5000".to_string(),
        data: json,
    };

    let response = request.send();
    print!("{}", response.data);

}
