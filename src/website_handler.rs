use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;

#[derive(Debug)]
pub struct WebsiteHandler {
    public_path : String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self{
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {

        let path = format!("{}/{}", self.public_path, file_path);
        
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(fs::canonicalize(&self.public_path).unwrap()){
                    fs::read_to_string(path).ok()
                }else{
                    println!("Directory traversal attack thwarted at {}.", file_path);
                    None
                }
            },
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response{
        dbg!(request);
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)),
                    None => Response::new(StatusCode::NotFound, self.read_file("/errorpages/404.html"))
                },
            },
            Method::HEAD => match request.path() {
                "/" => Response::new(StatusCode::Ok, None),
                path => match self.read_file(path) {
                    Some(_) => Response::new(StatusCode::Ok, None),
                    None => Response::new(StatusCode::NotFound, self.read_file("/errorpages/404.html"))
                },
            },
            Method::PUT | Method::DELETE | Method::PATCH => Response::new(StatusCode::MethodNotAllowed, self.read_file("/errorpages/503.html")),
            _ => Response::new(StatusCode::NotImplemented, self.read_file("/errorpages/503.html"))
        }
    }
}