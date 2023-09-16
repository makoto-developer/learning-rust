use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

fn main() {
    print!("boot rust server...\n");
    start_rust_server();
}

fn start_rust_server() {
    let server_port = 29000;
    let server_url = format!("127.0.0.1:{}", &server_port);
    let listener = TcpListener::bind(&server_url).unwrap();
    println!("Starting Rust TCP server URL: {}", &server_url);

    for _stream in listener.incoming() {
        println!("Connection established!");
        let stream = _stream.unwrap();
        handle_request(stream)

    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    // go 404 page if server recieve beside get method.
    if buffer.starts_with(get) {
        let mut file = File::open("src/index.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        println!("show 404 page.");
        let mut file = File::open("src/404.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}