use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("received request");
            }
            Err(e) => { 
                println!("Connection failed");
            }
        }
    }
}