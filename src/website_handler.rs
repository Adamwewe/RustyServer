use crate::http::{Request, StatusCode, Response, Method};
use std::fs;
use super::server::Handler;

pub struct WebsiteHandler{
    public_path : String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self{public_path}
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        fs::read_to_string(path).ok()

    }
}
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::Ok, Some("<h1> EWAAAAA </h1>".to_string()))

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/Adam" => Response::new(StatusCode::Ok, self.read_file("index_hello.html")),

                _ => Response::new(StatusCode::NotFound, None)
            }
            _ => Response::new(StatusCode::NotFound, None)
        }

    }
}