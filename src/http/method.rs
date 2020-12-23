use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,     //The GET method requests a representation of the specified resource. Requests using GET should only retrieve data.
    HEAD,    //The HEAD method asks for a response identical to that of a GET request, but without the response body.
    POST,    //The POST method is used to submit an entity to the specified resource, often causing a change in state or side effects on the server.
    PUT,     //The PUT method replaces all current representations of the target resource with the request payload.
    DELETE,  //The DELETE method deletes the specified resource.
    CONNECT, //The CONNECT method establishes a tunnel to the server identified by the target resource.
    OPTIONS, //The OPTIONS method is used to describe the communication options for the target resource.
    TRACE,   //The TRACE method performs a message loop-back test along the path to the target resource.
    PATCH,   //The PATCH method is used to apply partial modifications to a resource. 
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;