pub enum Statuscode {}

pub struct Response {
    statuscode: Statuscode,
    body: Option<String>,
}

impl Response {
    pub fn new(statuscode: Statuscode, body: Option<String>) -> Self{
        Response {statuscode, body}
    }
}