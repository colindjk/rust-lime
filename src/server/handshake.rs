use std::io::Error as IoError;

use futures::{Future, Poll};
use tokio_core::{net, io};

use envelope::{ LimeCodec, Session, SealedEnvelope as Envelope };
use super::{EnvStream};
use super::node::Authentication;

/// The trait used to produce an EnvStream.
///
/// By implementing the 'Handshake' trait, you also implement the 'Future'
/// trait, which is used to produce 'ClientConnection' structs. These are
/// then passed to an 'Authentication' struct.
pub trait Handshake {
    type Stream: EnvStream;
    type LimeCodec: io::Codec = LimeCodec;

    /// A 'Session' envelope is provided if a new one arrives, otherwise the
    /// value was simply poll'd.
    ///
    /// This trait will remain a bit "open", other parts of the project are a
    /// much higher priority than the handshake, so this trait will remain
    /// simple until further development can occur.
    fn update_handshake(&mut self, envelope: Option<<<Self as Handshake>::LimeCodec as io::Codec>::In>)
        -> Poll<Self::Stream, IoError>; // TODO: Fix this ambiguous Request / Response situation.
}

impl<S: EnvStream, C: io::Codec> Future for Handshake<Stream=S, LimeCodec=C> {
    type Item = Authentication<S>;
    type Error = IoError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

    }
}

