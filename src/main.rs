use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::str;

fn respond_hello_world(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    stream.write(response).expect("Write failed");
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).expect("Read failed");
    println!("{}", str::from_utf8(&buffer).unwrap());
    respond_hello_world(stream);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_request(stream)
                });
            }
            Err(e) => { 
                println!("Connection failed");
            }
        }
    }
}