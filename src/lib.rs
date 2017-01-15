#![feature(proc_macro, custom_attribute, custom_derive, plugin)]
#![feature(conservative_impl_trait)]
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
pub mod user;
//pub mod client; // temporary, the client will be back!
#[macro_use]
pub mod envelope; // protocol src
pub mod utils;

use envelope::SealedEnvelope as Envelope;

pub struct EnvelopeError {
    envelope: Envelope,
    kind: ErrorKind,
}

/// TODO: Put this together with the error kind && macros stuff.
enum ErrorKind {
    Handshake,
    Authentication,
}
