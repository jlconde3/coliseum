use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:5000").await?;

    // Write data to the server
    stream.write_all(b"Hello from client!").await?;

    
    let mut buf = [0; 1024];
    match stream.read(&mut buf).await {
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
    Ok(())
}
