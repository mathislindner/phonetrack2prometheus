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
use prometheus::{Encoder, TextEncoder, Counter, Opts, Registry, core::Collector};

fn handle_client(mut stream: std::net::TcpStream, data: Arc<Mutex<HashMap<String, Value>>>, request_counter: Counter) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    println!("Received request: {}", request);

    if request.starts_with("POST /api HTTP/1.1") {
        handle_post_api(request.to_string(), stream, data);
    } else if request.starts_with("GET /metrics HTTP/1.1") {
        handle_get_metrics(request.to_string(), stream, request_counter);
    }
}

fn handle_post_api(request: String, mut stream: std::net::TcpStream, data: Arc<Mutex<HashMap<String, Value>>>) {
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
    update_metrics(&data);
    println!("{:?}", *data);
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_get_metrics(_request: String, mut stream: std::net::TcpStream, request_counter: Counter) {
    request_counter.inc();

    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&request_counter.collect(), &mut buffer).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        buffer.len(),
        String::from_utf8(buffer).unwrap()
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    dotenv().ok();
    let ip = env::var("RUST_HOST").expect("IP_ADDRESS environment variable not set");
    let port = env::var("RUST_PORT").expect("PORT environment variable not set");
    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(&address).unwrap();
    let data = Arc::new(Mutex::new(HashMap::new()));

    let request_counter = Counter::with_opts(Opts::new("requests_total", "Total number of requests received")).unwrap();
    let registry = Registry::new();
    registry.register(Box::new(request_counter.clone())).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let data = Arc::clone(&data);
        let request_counter = request_counter.clone();

        thread::spawn(move || {
            handle_client(stream, data, request_counter);
        });
    }
}
