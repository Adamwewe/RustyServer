mod server;
mod http;

use http::Method;
use http::Request;
use server::Server;
use std::env;

use website_handler::WebsiteHandler;
mod website_handler;

// #[allow(dead_code)] // silence compiler warnings


fn main() {
    
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    let string = String::from("127.0.0.1:8080");
    let pub_path = env::var("PUBLIC_PATH").unwrap_or(default_path);


    let server = Server::new(string);
    server.run(WebsiteHandler::new(pub_path));

    let get : Method = Method::GET;
    let put : Method = Method::DELETE;

}
