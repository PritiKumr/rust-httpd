use std::net::{TcpListener, TcpStream, SocketAddr};
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

fn handle_cgi_script(request: httparse::Request, mut stream: TcpStream, client_addr: SocketAddr, req_path: &str) {

    let path_components: Vec<&str> = req_path.splitn(2, "/").collect();
    let default_path = "/";
    let (script_name, path_info) = (path_components.get(0).unwrap(), path_components.get(1).unwrap_or(&default_path));

    let client_ip = client_addr.ip().to_string();

    let meta_variables = build_cgi_meta_vars(&request, &client_ip, script_name, path_info);

    let mut command = Command::new(format!("cgi/{}", script_name));

    println!("{:?}", &meta_variables);
    build_environmental_variables(&mut command, meta_variables);


    
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

fn build_environmental_variables<'a>(command: &'a mut Command, meta_variables: Vec<(&'a str, &'a str)>) {
    for &tup in meta_variables.iter() {
        command.env(tup.0, tup.1);
    } 

    println!("{:?}", command);
}

fn build_cgi_meta_vars<'a>(request: &'a httparse::Request, client_ip: &'a String, script_name: &'a str, path_info: &'a str) -> Vec<(&'a str, &'a str)> {
    let mut headers = Vec::new();

    for (i, &item) in request.headers.iter().enumerate() {
        match &item.name {
            &"Authorization" => headers.push(("AUTH_TYPE", str::from_utf8(&item.value).unwrap())),
            &"Content-Length" => headers.push(("CONTENT_LENGTH", str::from_utf8(&item.value).unwrap())),
            &"Content-Type" => headers.push(("CONTENT_TYPE", str::from_utf8(&item.value).unwrap())),
            &"Host" => {
                let header_value = str::from_utf8(&item.value).unwrap();

                match header_value.find(':') {
                    Some(index) => {
                        headers.push(("SERVER_NAME", &header_value[..(index)]));
                        headers.push(("SERVER_PORT", &header_value[(index + 1)..]));
                    },
                    None => {
                        headers.push(("SERVER_NAME", header_value));
                    }
                }
            },
            _ => {},
        };
    };

    headers.push(("REMOTE_ADDR", &client_ip[..]));
    headers.push(("REMOTE_HOST", &client_ip[..]));

    headers.push(("REQUEST_METHOD", request.method.unwrap()));
    headers.push(("SCRIPT_NAME", script_name));

    match path_info.find('?') {
        Some(index) => {
            headers.push(("PATH_INFO", &path_info[..(index)]));
            headers.push(("QUERY_STRING", &path_info[(index + 1)..]));
        },
        None => {
            headers.push(("PATH_INFO", path_info));
        }
    };

    headers.push(("SERVER_PROTOCOL", "HTTP 1.1"));
    headers.push(("SERVER_SOFTWARE", "rust-httpd 0.1"));

    return headers;
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

fn read_request_head(stream: &TcpStream) -> Vec<u8> {
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

fn handle_request(mut stream: TcpStream, client_addr: SocketAddr) {
    let request_bytes = read_request_head(&stream);
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&request_bytes);
    println!("{:?}", req.headers);
    let body_length: u32 = match req.headers.iter().find(|&&header| header.name == "Content-Length") {
        Some(header) => str::from_utf8(header.value).unwrap().parse().unwrap(),
        None => 0,
    };

    // let request_body = read_request_body();

    match req.path {
        Some(path) => {
            if path.starts_with("/files") {
                serve_static_file(stream, &path[7..]);
            } else if path == "/hello" {
                respond_hello_world(stream);
            } else if path.starts_with("/cgi") {
                handle_cgi_script(req, stream, client_addr, &path[5..]);
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
    
    loop {    
        match listener.accept() {
            Ok((stream, addr)) => { thread::spawn(move || {
                    handle_request(stream, addr);
                })
            },
            Err(e) => { 
                thread::spawn(move || { 
                    println!("Connection failed: {:?}", e)
                })
            },
        };
    };
}