use anyhow::{anyhow, Result};
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub async fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80").await?;

    loop {
        match listener.accept().await {
            Ok((stream, _address)) => {
                tokio::spawn(handle_connection(stream));
            }
            Err(err) => {
                return Err(anyhow!("Error accepting connection: {}", err.to_string()));
            }
        }
    }
}

#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpRequest {
    fn new() -> Self {
        HttpRequest {
            method: String::new(),
            path: String::new(),
            version: String::new(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

async fn parse_http_request(stream: &mut TcpStream) -> Result<HttpRequest> {
    let mut request = HttpRequest::new();
    let mut reader = BufReader::new(stream);
    let mut first_line = String::new();

    reader.read_line(&mut first_line).await?;
    let parts: Vec<&str> = first_line.trim().split_whitespace().collect();
    if parts.len() != 3 {
        return Err(anyhow!("Invalid HTTP request line"));
    }

    request.method = parts[0].to_string();
    request.path = parts[1].to_string();
    request.version = parts[2].to_string();

    // Read headers
    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await?;

        // Empty line marks the end of headers
        if line.trim().is_empty() || bytes_read == 0 {
            break;
        }

        // Parse header
        if let Some((key, value)) = line.split_once(':') {
            request
                .headers
                .insert(key.trim().to_lowercase(), value.trim().to_string());
        }
    }

    // Read body if Content-Length is present
    if let Some(length) = request.headers.get("content-length") {
        if let Ok(length) = length.parse::<usize>() {
            let mut body = vec![0; length];
            reader.read_exact(&mut body).await?;
            request.body = body;
        }
    }

    Ok(request)
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let request = parse_http_request(&mut stream).await?;

    println!("Method: {}", request.method);
    println!("Path: {}", request.path);
    println!("Version: {}", request.version);
    println!("Headers: {:?}", request.headers);
    println!("Body length: {}", request.body.len());

    let response = create_http_response("200 OK", "text/plain", "Hello from the gateway!");
    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

fn create_http_response(status: &str, content_type: &str, body: &str) -> String {
    let response = format!(
        "HTTP/1.1 {}\r\n\
     Content-Type: {}\r\n\
     Content-Length: {}\r\n\
     Connection: close\r\n\
     \r\n\
     {}",
        status,
        content_type,
        body.len(),
        body
    );
    response
}
