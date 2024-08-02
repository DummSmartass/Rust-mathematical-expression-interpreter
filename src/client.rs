use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run_client(texts: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    for text in texts {
        // Send the text
        stream.write_all(text.as_bytes()).await?;
        stream.write_all(b"\n").await?; // Use newline as delimiter

        // Read the reversed response
        let mut response = vec![0; 1024];
        let n = stream.read(&mut response).await?;
        let response_text = String::from_utf8_lossy(&response[..n]).trim_end().to_string();

        println!("Received reversed text: {}", response_text);
    }

    Ok(())
}
