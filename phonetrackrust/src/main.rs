use std::net::TcpListener;
use std::env;
use dotenv::dotenv;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::Value;
use std::thread;


fn handle_client(mut stream: std::net::TcpStream, data: Arc<Mutex<HashMap<String, Value>>>) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    println!("Received request: {}", request);

    if request.starts_with("POST /api HTTP/1.1") {
        let mut headers = request.lines(); // Declare as mutable
        let content_length = headers
            .find(|line| line.starts_with("Content-Length:"))
            .and_then(|line| line.split(": ").nth(1))
            .and_then(|len| len.trim().parse::<usize>().ok())
            .unwrap_or(0);

        let body_start = request.find("\r\n\r\n").unwrap() + 4; // Skip the headers
        let body = &request[body_start..body_start + content_length];

        println!("Body to parse: '{}'", body);

        let json: Value = match serde_json::from_str(body.trim()) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                return;
            }
        };

        let mut data = data.lock().unwrap();
        data.insert("api_data".to_string(), json);
        println!("{:?}", *data);

        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}



fn main() {
    dotenv().ok();
    let ip = env::var("RUST_HOST").expect("IP_ADDRESS environment variable not set");
    let port = env::var("RUST_PORT").expect("PORT environment variable not set");
    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(&address).unwrap();
    let data = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let data = Arc::clone(&data);

        thread::spawn(move || {
            handle_client(stream, data);
        });
    }
}