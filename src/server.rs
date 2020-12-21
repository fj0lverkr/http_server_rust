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
            listener.accept();
        }
    }
}