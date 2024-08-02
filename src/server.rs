use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024]; // Buffer for incoming data

            loop {
                // Read data from the socket
                match socket.read(&mut buf).await {
                    Ok(0) => break, // Connection closed
                    Ok(n) => {
                        let received_data = String::from_utf8_lossy(&buf[..n]).trim_end().to_string();

                        //interpret
                        let result = match received_data.as_str().split(":")[0] {
                            "a" => "0",
                            "b" => "1",
                            "c" => "2",
                            _ => "fuck you"
                        };

                        //let reversed_data = received_data.chars().rev().collect::<String>();

                        // Write reversed data back to the socket
                        if let Err(e) = socket.write_all(result.as_bytes()).await {
                            println!("Failed to write to socket; error = {:?}", e);
                        }
                    }
                    Err(e) => {
                        println!("Failed to read from socket; error = {:?}", e);
                        break;
                    }
                }
            }
        });
    }
}
