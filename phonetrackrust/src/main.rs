use std::net::TcpListener;
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv::dotenv().ok();
    let ip = env::var("RUST_HOST").expect("IP_ADDRESS environment variable not set");
    let port = env::var("RUST_PORT").expect("PORT environment variable not set");
    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(&address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}