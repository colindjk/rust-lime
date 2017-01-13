use std::net::SocketAddr;
use std::convert::From;
use std::io;
use std::sync::Arc;

use futures::{stream, future, sync, Future, Stream, Sink, Poll};
use tokio_core::io::{Io, Framed};
use tokio_core::net::{TcpStream};
use tokio_service::{Service, NewService};

use envelope::{LimeCodec, EnvelopeStream, SealedEnvelope as Envelope};
use user::{User};

use super::NodeMap;

type FutEnvelope = Future<Item=Envelope, Error=io::Error>;

/// A client connection is created per incoming connection.
///
/// Field 'stream' is the 'Io' object used for client communication.
/// Field 'user' pertain to potentially logged in user.
/// The client connection will be split once authenticated via 'Session'
/// envelopes.
pub struct ClientConnection<S> {
    inner: S,
}

/// Implementation
impl<S> ClientConnection<S>
    where S: Stream<Item=Envelope> + Sink<SinkItem=Envelope>
{
    pub fn new(io: S) -> Self { ClientConnection { inner: io } }

    /// The task of responding to a request. 
    pub fn respond(req: Envelope) -> Box<FutEnvelope> {
        unimplemented!()
    }
}

/// Service implementation for the 'ClientConnection' struct.
impl<S> Service for ClientConnection<S>
    where S: Stream<Item=Envelope> + Sink<SinkItem=Envelope>
{
    type Request = Envelope;
    type Response = Envelope;
    type Error = io::Error;
    type Future = Box<FutEnvelope>;

    fn call(&self, req: Envelope) -> Self::Future {
        unimplemented!()
    }
}

/// After a split, ClientSession is created which will be the recieving end of
/// a connection.
///
/// Created as a part of a succesful login.
pub struct ClientSession<S> {
    inner: sync::BiLock<ClientConnection<S>>,
    user: User,
    peers: Arc<NodeMap<S>>,
}

impl<S> ClientSession<S>
    where S: Stream<Item=Envelope> + S: Sink<SinkItem=Envelope>
{
    pub fn new(io: S) -> Self {
        panic!()
    }
}

/// Designed to make it easier to send over a connection / channel.
/// Not sure what else.
pub struct ClientSink<S> {
    inner: stream::SplitSink<S>,
}

impl<S> ClientSink<S>
    where S: Sink<SinkItem=Envelope>
{
    pub fn new(io: S) -> Self {
        panic!()
    }
}

//impl Service 

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

