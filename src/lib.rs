#![feature(proc_macro, custom_attribute, custom_derive, plugin)]
#![plugin(serde_derive)]

extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_proto;
extern crate maud;

extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;
#[macro_use]
extern crate serde_derive;

pub mod net;
//pub mod client; // temporary, the client will be back!
#[macro_use]
pub mod envelope;
pub mod utils;

