extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service;
extern crate tokio_proto;
extern crate tiny_http;
//extern crate protocol_manager;

//futures = "0.1.4"
//tokio-core = "0.1.1"
//tokio-service = { git = "https://github.com/tokio-rs/tokio-service" }
//tokio-proto= { git = "https://github.com/tokio-rs/tokio-proto" }
//tiny-http = "0.5"

use tiny_http::{ Server, Response };
use tokio::reactor;

fn main() {
    let server = tiny_http::Server::http("0.0.0.0:3000").unwrap();

    for request in server.incoming_requests() {
        println!("Request:\nMethod: {:?}\nURL: {:?}\nHeaders: {:?}",
            request.method(),
            request.url(),
            request.headers());

        let response = Response::from_string("Hello : dude");
        request.respond(response);
    }
}

