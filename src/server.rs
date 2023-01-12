use std::net::TcpListener;
// use crate::http::ParseError;
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};

pub trait Handler{
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }

}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {  
        Self {addr}
    }

    pub fn run(&self, mut handler: impl Handler)  {
        println!("listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr)
            .unwrap();  // to refactor with uwnrap_or_else()

        loop {
            match listener.accept(){
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // generate 1024 bytes so we fill with garbage
                    match stream.read(&mut buffer) {//dangerous but more than enough to handle the requests we have
                        Ok(_) => {
                            println!("Received request {:?}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {

                                    handler.handle_request(&request)
                                    // dbg!(request);
                                    // Response::new(StatusCode::Ok, 
                                    //     Some("<h1> Hey There Stranger :) </h1>".to_string()),
                                    // )
                                }
                                Err(e) => {
                                    // println!("Failed to parse request {}", e);
                                    // Response::new(StatusCode::BadRequest, None)
                                    handler.handle_bad_request(&e)
                                }

                            };
                            // let res : &Result<Request, _> = &buffer[..].try_into();
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response {}", e);
                            }

                        }
                        Err(e) => println!("An error has occured: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection, reason: {}", e),
        }
    }
    }
}

