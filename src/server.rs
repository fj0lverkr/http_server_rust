use crate::http::{Request, Response, StatusCode, ParseError};
use crate::threading::ThreadPool;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request; {}.", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
    poolsize: usize,
}

impl Server {

    pub fn new(addr: String, poolsize: usize) -> Self {
        Server {
            addr: addr,
            poolsize: poolsize,
        }
    }

    pub fn run(self, mut handler: impl Handler + Send + 'static) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}, using {} threads.", &self.addr, self.poolsize);
        let pool = ThreadPool::new(self.poolsize);
        loop{
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    pool.execute(move || {
                        println!("Connection accepted from {}.", addr);
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                println!("Received request: {}.", String::from_utf8_lossy(&buffer));
                                let response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => handler.handle_request(&request),
                                    Err(e) => handler.handle_bad_request(&e)
                                };
                                if let Err(e) = response.send(&mut stream){
                                    println!("Failed to send response; {}.", e);
                                }
                            }
                            Err(e) => println!("Failed to read from connection: {}", e),
                        }
                    });
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}