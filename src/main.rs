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

struct Server {
    thread_pool: CpuPool
}

#[derive(RustcEncodable)]
struct Message {
    id: i32,
    randomNumber: i32,
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Response, io::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let msg = Message {
		                id: 1,
		                randomNumber: 22,
			            };

        let json = rustc_serialize::json::encode(&msg).unwrap();
        let mut response = Response::new();
        response.header("Content-Type", "application/json");
        response.body(&json);
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