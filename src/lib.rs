extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service;
extern crate tokio_proto;
extern crate tiny_http;
extern crate maud;

mod server;
mod client;
mod frames;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
