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
    }
}