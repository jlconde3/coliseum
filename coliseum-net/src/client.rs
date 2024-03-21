mod lib;

use lib;


fn main() {
    let mut client = Client {
        addr: "127.0.0.1:5002".to_string(),
        register_addr: "127.0.0.1:5000".to_string(),
        clients: Arc::new(RwLock::new(HashSet::new())),
        storage: Arc::new(RwLock::new(Vec::new())),
    };

    println!("-----------CREANDO UN ITEM------------");

    let item = client
        .send_item("ESTO ES UN ITEM ENVIADO".to_string())
        .await;

    println!("{}", &item.id);
    println!("{}", item.timestamp);
    println!("{}", item.content);

    println!("-----------RECUPERANDO UN ITEM------------");

    let item = client.retrive_item(item.id).await;
    println!("{}", &item.id);
    println!("{}", item.timestamp);
    println!("{}", item.content);
}
