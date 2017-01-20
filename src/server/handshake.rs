use std::io::Error as IoError;

use futures::{Future, Poll, Stream, Sink, Async};
use tokio_core::{net, io};

use envelope::{ LimeCodec, Session, Envelope };
use super::{EnvStream};
use super::node::Authentication;

/// A future which evaluates to an `EnvStream`.
///
/// By implementing the 'Handshake' trait, you also implement the 'Future'
/// trait, which is used to produce 'ClientConnection' structs. These are
/// then passed to an 'Authentication' struct.
pub trait Handshake {
    type Stream: EnvStream;

    fn take_stream(&mut self, tcp: net::TcpStream);

    fn drop_stream(&mut self);

    /// A 'Session' envelope is provided if a new one arrives, otherwise the
    /// value was simply poll'd.
    ///
    /// This trait will remain a bit "open", other parts of the project are a
    /// much higher priority than the handshake, so this trait will remain
    /// simple until further development can occur.
    fn update_handshake(&mut self) -> Poll<Self::Stream, IoError>;
    // TODO: Fix this ambiguous Request / Response situation.
}

/// TODO: Change this to a Stream implementation
impl<S: EnvStream> Future for Handshake<Stream=S> {
    type Item = S;
    type Error = IoError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.update_handshake()
    }
}

