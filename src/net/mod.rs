use std::io;

use std::net::SocketAddr;
use std::io;

use futures::{stream, Stream, future, Future, Poll, BoxFuture};
use tokio_core::net::{TcpListener};

//use tokio_core::io::{Io, Framed};
//use envelope::{LimeCodec, SealedEnvelope as Envelope};

/// Initial server which can used to accept and work with connections.
/// Generally it will be used to accept incoming connections.
pub struct LimeServer {
    addr: SocketAddr,
    listener: TcpListener,
}

/// Implementation of the LimeServer. Provides functionality for accepting
/// connections, and providing Nodes in an un-authenticated state.
impl LimeServer {
    /// Creates a new server from a TcpListener.
    /// TODO: Try to figure out Websockets, HTTP etc.
    fn bind(addr: &SocketAddr, handle: &Handle) -> io::Result<Self> {

    }
}

pub struct Nodes {
    inner: Server,
}

/// A stream of incoming connections, which are evaluated to be Nodes.
impl Stream for Nodes {
    type Item = Node;
    type Error = io::Error;

    /// Attempt to make a connection to a Node.
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

    }
}

//pub struct LimeProtocol;

///// TODO: Make sure the Request gets to the right place (user / db?), then start
///// to worry about the actual response (what the future returns).

///// This, uh... 'hooks up the codec'. Yeah.
//impl<T: Io + 'static> ServerProto<T> for LimeProtocol {
    //type Request = Envelope;
    //type Response = Envelope;
    //type Error = io::Error;
    //type Transport = Framed<T, LimeCodec>;
    //type BindTransport = io::Result<Framed<T, LimeCodec>>;

    //fn bind_transport(&self, io: T) -> io::Result<Framed<T, LimeCodec>> {
        //Ok(io.framed(LimeCodec))
    //}
//}

//impl Service for Server {
    //type Request = Envelope;
    //type Response = Envelope;
    //type Error = io::Error;
    //type Future = BoxFuture<Envelope, io::Error>;

    ///// For now we'll just return the message that was sent
    //fn call(&self, req: Envelope) -> Self::Future {
        //future::finished(req).boxed()
    //}
//}

