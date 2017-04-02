use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn respond_hello_world(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    stream.write(response).expect("Write failed");
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    respond_hello_world(stream)
                });
            }
            Err(e) => { 
                println!("Connection failed");
            }
        }
    }
}