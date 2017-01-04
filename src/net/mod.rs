pub mod node;

use std::net::SocketAddr;
use std::convert::From;
use std::io;

use futures::{stream, Stream, future, Future, Poll, BoxFuture};

use tokio_core::io::{Io};
use tokio_core::net;
use tokio_core::reactor::{Handle};

// TODO : Refactor to make sense
pub use self::node::*;

/// Generally it will be used to accept incoming connections.
/// 'L' will be any type of listener, which produces a stream of
/// ClientConnection structs.
pub struct LimeServer<L> {
    addr: Option<SocketAddr>,
    incoming: L,
}

/// Implementation of the LimeServer. Provides functionality for accepting
/// connections, and providing Nodes in an un-authenticated state.
impl<L, T> LimeServer<L> where L: Stream<Item=ClientConnection<T>> {
    /// Creates a new server from a TcpListener.
    /// TODO: Try to figure out Websockets, HTTP etc.
    fn bind<F>(addr: &SocketAddr, handle: &Handle) -> io::Result<Self>
    {
        let lis = net::TcpListener::bind(addr, handle)?;
        let inc = lis.incoming().map(|(s, _)| ClientConnection::from(s));
        Ok(LimeServer::<stream::Map<net::Incoming, F>> {
        //Ok(LimeServer {
            addr: Some(addr.clone()),
            incoming: inc,
        })
    }

}

