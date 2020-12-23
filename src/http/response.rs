use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    statuscode: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(statuscode: StatusCode, body: Option<String>) -> Self{
        Response {statuscode, body}
    }
}