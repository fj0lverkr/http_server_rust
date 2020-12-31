use super::StatusCode;
use std::io::{Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    statuscode: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(statuscode: StatusCode, body: Option<String>) -> Self{
        Response {statuscode, body}
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{
        
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}",
        self.statuscode,
        self.statuscode.reason_phrase(),
        body
        
    )}
}