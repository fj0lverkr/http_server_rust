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
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method,
}

enum Method {
    GET,
    //The GET method requests a representation of the specified resource. Requests using GET should only retrieve data.
    HEAD,
    //The HEAD method asks for a response identical to that of a GET request, but without the response body.
    POST,
    //The POST method is used to submit an entity to the specified resource, often causing a change in state or side effects on the server.
    PUT,
    //The PUT method replaces all current representations of the target resource with the request payload.
    DELETE,
    //The DELETE method deletes the specified resource.
    CONNECT,
    //The CONNECT method establishes a tunnel to the server identified by the target resource.
    OPTIONS,
    //The OPTIONS method is used to describe the communication options for the target resource.
    TRACE,
    //The TRACE method performs a message loop-back test along the path to the target resource.
    PATCH,
    //The PATCH method is used to apply partial modifications to a resource. 

}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/