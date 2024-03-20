use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:5000").await?;

    // Write data to the server
    stream.write_all(b"Hello from client!").await?;
    Ok(())
}
