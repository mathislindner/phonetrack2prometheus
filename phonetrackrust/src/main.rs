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
use prometheus::{Encoder, TextEncoder};

fn handle_client(mut stream: std::net::TcpStream, data: Arc<Mutex<HashMap<String, Value>>>) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    println!("Received request: {}", request);

    if request.starts_with("POST /api HTTP/1.1") {
        handle_post_api(request.to_string(), stream, data);
    } else if request.starts_with("GET /metrics HTTP/1.1") {
        handle_get_metrics(request.to_string(), stream, data);
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

    //println!("Body to parse: '{}'", body);

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


fn handle_get_metrics(_request: String, mut stream: std::net::TcpStream, data: Arc<Mutex<HashMap<String, Value>>>) {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    let data = data.lock().unwrap();

    let key_rename_map = HashMap::from([
        ("_type", ("phonetrack_type", "Type of the phonetrack data")),
        ("acc", ("phonetrack_accuracy", "Accuracy of the phonetrack data")),
        ("alt", ("phonetrack_altitude", "Altitude of the phonetrack data")),
        ("batt", ("phonetrack_battery", "Battery level of the phonetrack device")),
        ("lat", ("phonetrack_latitude", "Latitude of the phonetrack data")),
        ("lon", ("phonetrack_longitude", "Longitude of the phonetrack data")),
        ("tst", ("phonetrack_timestamp", "Timestamp of the phonetrack data")),
        ("vel", ("phonetrack_velocity", "Velocity of the phonetrack data")),
        ("tid", ("phonetrack_tracker_id", "Tracker ID of the phonetrack data")),
    ]);

    let mut metrics = Vec::new();
    if let Some(api_data) = data.get("api_data") {
        if let Some(obj) = api_data.as_object() {
            for (key, value) in obj {
                let key_str = key.as_str();
                let (renamed_key, description) = key_rename_map.get(key_str).map(|v| *v).unwrap_or((key_str, "No description available"));
                if let Some(val) = value.as_str() {
                    metrics.push(format!("# HELP {} {}\n# TYPE {} gauge\n{} {}", renamed_key, description, renamed_key, renamed_key, val));
                } else if let Some(val) = value.as_i64() {
                    metrics.push(format!("# HELP {} {}\n# TYPE {} gauge\n{} {}", renamed_key, description, renamed_key, renamed_key, val));
                }
            }
        }
    }

    let metrics_str = metrics.join("\n");
    buffer.extend_from_slice(metrics_str.as_bytes());
    encoder.encode(&[], &mut buffer).unwrap();

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

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let data = Arc::clone(&data);

        thread::spawn(move || {
            handle_client(stream, data);
        });
    }
}
