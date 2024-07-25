use tokio::io::{AsyncReadExt, AsyncWriteExt};
// client.rs
use tokio::net::TcpStream;

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let request = b"a";

    // Send the request
    stream.write_all(request).await?;

    let mut response = [0; 1];

    // Read the response
    stream.read_exact(&mut response).await?;

    if response[0] == b'b' {
        println!("Received expected response: b");
    } else {
        println!("Unexpected response: {:?}", response);
    }

    Ok(())
}
