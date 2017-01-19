use std::net::SocketAddr;
use std::convert::From;
use std::io::Error as IoError;
use std::sync::Arc;

use futures::{stream, future, sync, Future, BoxFuture, Stream, Sink, Async, Poll};
use tokio_core::io::{Io, Framed};
use tokio_core::net::{TcpStream};
use tokio_service::{Service, NewService};

use envelope::{Node, LimeCodec, EnvelopeStream, SealedEnvelope as Envelope,
    Session,
};
use envelope::session::{
    EncryptionOptions,
    CompressionOptions,
    SchemeOptions,
};
use user::{User};

use super::{NodeMap, EnvStream};

type FutEnvelope = Box<Future<Item=Envelope, Error=IoError> + Send>;

/// A client connection is created per incoming connection.
///
/// Field 'stream' is the 'Io' object used for client communication.
/// Field 'user' pertain to potentially logged in user.
/// The client connection will be split once authenticated via 'Session'
/// envelopes.
pub struct ClientConnection<S> { inner: S }

/// Implementation
/// TODO: Create an error type for connection stuff.
/// Note:
/// -   Either have two error types, one for critical errors w/ system crash & bang
/// -   Or one error type and pass it up or handle it / panic when deemed appropriate.
impl<S: EnvStream> ClientConnection<S> {
    pub fn new(io: S) -> Self { ClientConnection { inner: io } }
}

impl<S: EnvStream> Stream for ClientConnection<S> {
    type Item = Envelope;
    type Error = IoError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}

/// Future which will evaluate to a properly configured connection.
///
/// Handshake, also known as the 'Negotiating' phase of the overall session.
pub struct TcpHandshake {
    conn: Option<Framed<TcpStream, LimeCodec>>,
    user_id: Option<Node>,
    encryption: EncryptionOptions,
    compression: CompressionOptions,
}

impl Service for TcpHandshake {
    type Request = Session;
    type Response = Session;
    type Error = IoError;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}

//impl Future for Handshake {
    //type Item = Authentication<EnvStream>;
    //type Error = IoError;

    ///// This is where some sort of database query would occur.
    //fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        //unimplemented!()
    //}
//}

/// This will be the future representing the authentication process.
///
/// TODO: Include a password attempt future which will be a 'helper future' of sorts
pub struct Authentication<S> {
    conn: Option<ClientConnection<S>>,
    peers: NodeMap<S>, // TODO: Make this a ref to something more pertinent.
    user_id: Option<Node>,
    password: String,
    authenticated: bool,
    scheme: SchemeOptions,
}

impl<S> Service for Authentication<S> {
    type Request = Session;
    type Response = Session;
    type Error = IoError;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}

impl<S> Authentication<S> {
    /// TODO: Implement an authentication update thingy.
    pub fn update_auth(&mut self, envelope: Session) {
        self.authenticated = true;
    }
}

impl<S: EnvStream> Future for Authentication<S> {
    type Item = (ClientSink<S>, ClientSession<S>);
    type Error = IoError;

    /// This is where some sort of database query would occur.
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.conn.as_mut().unwrap().poll() {
            Ok(Async::Ready(Some(env))) => {
                if let Envelope::Session(s) = env {
                    self.update_auth(s);
                } else {
                    panic!("Received non-session envelope \
                           during session authentication.");
                };
            },
            Ok(Async::Ready(None)) => panic!("Implement EOF during authentication"),
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(_) => panic!("Error envelope from stream \
                                    during Authentication"),
        };

        if self.authenticated {
            let conn = self.conn.take().unwrap();
            let (sink, session) = sync::BiLock::new(conn);
            Ok(Async::Ready((
                ClientSink { inner: sink, },
                ClientSession {
                    inner: session,
                    user_id: self.user_id.take().unwrap(),
                    user: User,
                    peers: self.peers.clone(),
                }
            )))
        } else {
            Ok(Async::NotReady)
        }
    }
}

/// After a split, ClientSession is created which will be the recieving end of
/// a connection.
///
/// Created as a part of a succesful login.
pub struct ClientSession<S> {
    inner: sync::BiLock<ClientConnection<S>>,
    user_id: Node,
    user: User,
    peers: NodeMap<S>,
}

/// Service implementation for the 'ClientSession' struct.
///
/// ClientSession implements the Service trait to avoid having a blocking event
/// occur on the stream of incoming messages when not necessary.
impl<S> Service for ClientSession<S> {
    type Request = Envelope;
    type Response = Envelope;
    type Error = IoError;
    type Future = Box<FutEnvelope>;

    fn call(&self, req: Envelope) -> Self::Future {
        unimplemented!()
    }
}

impl<S> ClientSession<S> {
    pub fn new(io: S) -> Self {
        panic!()
    }
}

/// Designed to make it easier to send over a connection / channel.
/// Not sure what else.
pub struct ClientSink<S> {
    inner: sync::BiLock<ClientConnection<S>>,
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
impl<S: EnvStream> From<S> for ClientConnection<S> {
    fn from(io: S) -> Self {
        ClientConnection { inner: io }
    }
}

/// 'Io'
impl From<(TcpStream, SocketAddr)>
        for ClientConnection<EnvelopeStream<TcpStream>> {
    fn from(connection: (TcpStream, SocketAddr)) -> Self {
        let (stream, _) = connection;
        let stream = stream.framed(LimeCodec);
        ClientConnection { inner: stream }
    }
}

