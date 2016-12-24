#![feature(proc_macro, custom_attribute, custom_derive, plugin)]
#![plugin(serde_derive)]

extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service;
extern crate tokio_proto;
extern crate maud;

extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;
#[macro_use]
extern crate serde_derive;

pub mod server;
pub mod client;
pub mod lime;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
