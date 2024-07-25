use tokio::io::{AsyncReadExt, AsyncWriteExt};
// server.rs
use tokio::net::TcpListener;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1];

            // Read data from the socket
            match socket.read_exact(&mut buf).await {
                Ok(_) => {
                    if buf[0] == b'a' {
                        // Write response back to the socket
                        if let Err(e) = socket.write_all(b"b").await {
                            println!("Failed to write to socket; error = {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to read from socket; error = {:?}", e);
                }
            }
        });
    }
}
