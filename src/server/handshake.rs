use std::io;

use futures::{Future, Poll};
use tokio_core::net;

use envelope::{ Session, SealedEnvelope as Envelope };
use super::{EnvStream};
use super::node::Authentication;

/// The trait used to produce an EnvStream.
///
/// By implementing the 'Handshake' trait, you also implement the 'Future'
/// trait, which is used to produce 'ClientConnection' structs. These are
/// then passed to an 'Authentication' struct.
pub trait Handshake {
    type Stream: EnvStream;

    fn update_handshake(&mut self, envelope: Session)
        -> Poll<Self::Stream, io::Error>;
}

impl<S: EnvStream> Future for Handshake<Stream=S> {
    type Item = Authentication<S>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        unimplemented!()
    }
}

