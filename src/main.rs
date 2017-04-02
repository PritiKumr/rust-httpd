use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::Read;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).expect("Read failed");
    println!("{:?}", buffer);
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("received request");
                handle_client(stream);
                break;
            }
            Err(e) => { 
                println!("Connection failed");
            }
        }
    }
}