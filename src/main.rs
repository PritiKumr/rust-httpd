extern crate futures;
extern crate futures_cpupool;
extern crate rustc_serialize;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;

use futures::{future, BoxFuture, Future};
use futures_cpupool::CpuPool;
use tokio_minihttp::{Request, Response};
use tokio_proto::TcpServer;
use tokio_service::Service;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Server {
    thread_pool: CpuPool
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Response, io::Error>;

    fn call(&self, req: Request) -> Self::Future {
    		let mut file_path = String::from(req.path());
    		file_path.remove(0);
    		let file = match File::open(file_path) {
                Ok(file) => file,
                Err(err) => File::open("404.html").expect("404.html file missing!"),
            };
			let mut buf_reader = BufReader::new(file);
			let mut contents = String::new();
			buf_reader.read_to_string(&mut contents);

        // let json = rustc_serialize::json::encode(&contents).unwrap();
        let mut response = Response::new();
        response.body(&contents);
        future::ok(response).boxed()
    }
}

fn main() {
    let addr = "127.0.0.1:8888".parse().unwrap();
    let thread_pool = CpuPool::new(10);

    TcpServer::new(tokio_minihttp::Http, addr).serve(move || {
        Ok(Server {
            thread_pool: thread_pool.clone(),
        })
    })
}