use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
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

fn handle_cgi_script(request: httparse::Request, mut stream: TcpStream, path: &str) {
    for (i, &item) in request.headers.iter().enumerate() {
        println!("{} {:?}", &item.name, str::from_utf8(&item.value));
        match &item.name {
            Some(expr) => expr,
            None => expr,
        }
    }
    // build_cgi_headers(request.headers);

    let mut command = Command::new(format!("cgi/{}", path));
    command.env("REQUEST_METHOD", request.method.unwrap());
    // command.env()

    match command.output() {
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

// fn build_cgi_headers<'a>(headers: &'a [httparse::Header]) -> &'a[(&'a str, &'a str)] {
//     println!("{:?} {}",str::from_utf8(headers[1].value), headers[1].name);
//     let name = headers[1].name;
//     let value = str::from_utf8(headers[1].value).unwrap();
//     &[(name, value)]
// }

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

fn read_request(stream: &TcpStream) -> Vec<u8> {
    let mut reader = BufReader::new(stream);
    let mut buff = Vec::new();
    let mut read_bytes = reader.read_until(b'\n', &mut buff).unwrap();
    while read_bytes > 0 {
        read_bytes = reader.read_until(b'\n', &mut buff).unwrap();
        if read_bytes == 2 && &buff[(buff.len()-2)..] == b"\r\n" {
            break;
        }
    }
    return buff;
}

fn handle_request(mut stream: TcpStream) {
    let request_bytes = read_request(&stream);
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&request_bytes);

    match req.path {
        Some(path) => {
            if path.starts_with("/files") {
                serve_static_file(stream, &path[7..]);
            } else if path == "/hello" {
                respond_hello_world(stream);
            } else if path.starts_with("/cgi") {
                handle_cgi_script(req, stream, &path[5..]);
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
                    handle_request(stream);
                });
            }
            Err(_) => { 
                println!("Connection failed");
            }
        }
    }
}