pub mod node;

use std::net::SocketAddr;
use std::convert::{From};
use std::io;

use futures::{stream, Stream, future, Future, Poll, BoxFuture};

use tokio_core::io::{Framed, Io};
use tokio_core::net;
use tokio_core::reactor;

// the locals
use envelope::{LimeCodec, EnvelopeStream};

// TODO : Refactor to make sense
pub use self::node::*;

/// Generally it will be used to accept incoming connections.
/// 'L' will be any type of listener, which produces a stream of
/// ClientConnection structs.
pub struct LimeServer<L> {
    addr: SocketAddr,
    incoming: L,
}

/// Implementation of the LimeServer. Provides functionality for accepting
/// connections, and providing Nodes in an un-authenticated state.
impl<L, T> LimeServer<L> where L: Stream<Item=ClientConnection<T>> {
    /// Creates a new server from a TcpListener.
    /// TODO: Try to figure out Websockets, HTTP etc.
    pub fn new(addr: &SocketAddr, io: L) -> Self {
        LimeServer {
            addr: addr.clone(),
            incoming: io,
        }
    }
}

/// TODO: Figure out how returning a future would help.
pub fn example(addr: &SocketAddr, listen: net::TcpListener)
        -> LimeServer
        <impl Stream<Item=ClientConnection<EnvelopeStream<net::TcpStream>>>>
{
    LimeServer::new(addr, listen.incoming().map(
        |(stream, _)| ClientConnection::from(stream)
    ))
}

