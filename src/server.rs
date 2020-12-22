use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

//implement the structure
impl Server {

    //constructor
    pub fn new(addr: String) -> Self {
        Server {
            addr: addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Connection accepted from {}.", addr);
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer);
                },
                Err(e) => {
                    println!("Error connecting: {}.", e);
                    continue;
                }
            }
        }
    }
}