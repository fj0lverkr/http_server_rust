fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

//structure Server
struct Server {
    addr: String,
}

//implement the structure
impl Server {

    //constructor
    fn new(addr: String) -> Self {
        Server {
            addr: addr
        }
    }

    fn run(self) {
        println!("{}", self.addr);
    }
}