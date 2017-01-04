use std::convert::From;
use std::io;

use futures::{stream, Stream, Sink};
use tokio_core::io::{Io, Framed};
use tokio_core::net::{TcpStream};

use envelope::{LimeCodec, EnvelopeStream, SealedEnvelope as Envelope};
use user::{User};

/// A client connection is created per incoming connection.
///
/// Field 'stream' is the 'Io' object used for client communication.
/// Field 'user' pertain to potentially logged in user.
/// The client connection will be split once authenticated via 'Session'
/// envelopes.
pub struct ClientConnection<T> {
    inner: T,
    user: Option<User>,
}

impl<T> ClientConnection<T>
    where T: Stream<Item=Envelope> + Sink<SinkItem=Envelope>
{
    pub fn new(io: T) -> io::Result<Self> {
        panic!()
    }
}

pub struct ClientStream<T> {
    inner: stream::SplitStream<EnvelopeStream<T>>,
}

pub struct ClientSink<T> {
    inner: stream::SplitSink<EnvelopeStream<T>>,
}

/// 'Io'
impl<S> From<S> for ClientConnection<EnvelopeStream<S>> where S: Io {
    fn from(io: S) -> Self {
        let stream = io.framed(LimeCodec);
        ClientConnection {
            inner: stream,
            user: None,
        }
    }
}

