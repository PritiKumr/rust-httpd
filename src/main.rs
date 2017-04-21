use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::str;
use std::fs::File;
use std::process::Command;

extern crate httparse;

fn respond_hello_world(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    stream.write(response).expect("Write failed");
}

fn serve_static_file(mut stream: TcpStream, path: &str) {
    let mut file = match File::open(format!("www/{}", path)) {
        Ok(file) => file,
        Err(_) => File::open("404.html").expect("404.html file missing!"),
    };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Read failed");

    stream.write(&buffer).expect("Write failed");
}

fn handle_cgi_script(mut stream: TcpStream, path: &str) {
    // let output = Command::new(format!("cgi/{}", path))
    match Command::new(format!("cgi/{}", path)).output() {
        Ok(output) => {
            if output.status.success() {
                stream.write(&output.stdout).expect("Command failed");
            } else {
                stream.write(&output.stderr).expect("Stderr");
            }
        },
        Err(_) => {
            respond_error(stream);
        }
    }               
}

fn respond_error(mut stream: TcpStream) {
    let response = b"HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>500 - Server Error</body></html>\r\n";
    stream.write(response).expect("Write failed");
}

fn respond_file_not_found(mut stream: TcpStream) {
    let response = b"HTTP/1.1 404 File Not Found\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>404 - File Not Found</body></html>\r\n";
    stream.write(response).expect("Write failed");
}

fn request_url(buffer: &[u8]) -> Option<&str> {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    match req.parse(&buffer) {
        Ok(_) => {
            match req.path {
                Some(ref path) => {
                    return Some(path);
                },
                None => {
                  return None;  
                }
            }
        },
        Err(_) => {
            return None;
        }
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).expect("Read failed");

    match request_url(&buffer) {
        Some(path) => {
            if path.starts_with("/files") {
                serve_static_file(stream, &path[7..]);
            } else if path == "/hello" {
                respond_hello_world(stream);
            } else if path.starts_with("/cgi") {
                handle_cgi_script(stream, &path[5..]);
            } else {
                respond_file_not_found(stream);
            }
        },
        None => {
            respond_error(stream);
        }
    };
    
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
            Err(_) => { 
                println!("Connection failed");
            }
        }
    }
}