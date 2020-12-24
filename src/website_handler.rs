use super::server::Handler;
use super::http::{Request, Response, StatusCode};

#[derive(Debug)]
pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response{
        dbg!(request);
        Response::new(
            StatusCode::Ok,
            Some("<h1>It works!</h1>".to_string())
        )
    }
}