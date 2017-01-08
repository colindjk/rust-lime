use std::net::SocketAddr;
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
    pub fn new(io: T) -> Self {
        panic!()
    }
}

/// After a split, ClientStream is created which will be the recieving end of
/// a connection.
pub struct ClientStream<T> {
    inner: stream::SplitStream<T>,
}

impl<T> ClientStream<T>
    where T: Stream<Item=Envelope>
{
    pub fn new(io: T) -> Self {
        panic!()
    }
}

/// Designed to make it easier to send over a connection / channel.
/// Not sure what else.
pub struct ClientSink<T> {
    inner: stream::SplitSink<T>,
}

impl<T> ClientSink<T>
    where T: Sink<SinkItem=Envelope>
{
    pub fn new(io: T) -> Self {
        panic!()
    }
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

/// 'Io'
impl From<(TcpStream, SocketAddr)>
        for ClientConnection<EnvelopeStream<TcpStream>> {
    fn from(connection: (TcpStream, SocketAddr)) -> Self {
        let (stream, _) = connection;
        let stream = stream.framed(LimeCodec);
        ClientConnection {
            inner: stream,
            user: None,
        }
    }
}

