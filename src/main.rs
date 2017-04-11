use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::str;

extern crate httparse;

use httparse::EMPTY_HEADER;
use httparse::Request;


fn respond_hello_world(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    stream.write(response).expect("Write failed");
}

fn request_url(buffer: &[u8]) -> Option<&str> {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    match req.parse(&buffer) {
        Ok(result) => {
            match req.path {
                Some(ref path) => {
                    return Some(path);
                },
                None => {
                  return None;  
                }
            }
        },
        Err(msg) => {
            return None;
        }

    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).expect("Read failed");

    match request_url(&buffer).unwrap() {
        "/hello" => respond_hello_world(stream),
        _ => println!("Ain't special"),
    }
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