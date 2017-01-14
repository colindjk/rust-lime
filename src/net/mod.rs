pub mod node;

use std::net::SocketAddr;
use std::convert::{From};
use std::io;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::{stream, Stream, Sink, future, Future, Poll, BoxFuture};

use tokio_core::io::{Framed, Io};
use tokio_core::net;
use tokio_core::reactor;

// the locals
use envelope::{Node, LimeCodec, EnvelopeStream, SealedEnvelope as Envelope};

// TODO : Refactor to make sense
pub use self::node::*;

// TODO: Put a Mutex around that ClientSink!
type NodeMap<S> = HashMap<Node, ClientSink<S>>;
type ArcMut<T> = Arc<Mutex<T>>;

/// Generally it will be used to accept incoming connections.
/// 'L' will be any type of listener, which produces a stream of
/// ClientConnection structs.
///
/// TODO: Figure out a way to handle online users, is a HashMap optimal?
pub struct LimeServer<S> {
    addr: SocketAddr,
    users: Arc<NodeMap<S>>,
}

/// Implementation of the LimeServer. Provides functionality for accepting
/// connections, and providing Nodes in an un-authenticated state.
impl<S> LimeServer<S>
    where S: Stream<Item=Envelope> + Sink<SinkItem=Envelope>
{
    /// Creates a new server from a TcpListener.
    /// TODO: Try to figure out Websockets, HTTP etc.
    pub fn new(addr: &SocketAddr) -> Self {
        LimeServer {
            addr: addr.clone(),
            users: Arc::new(HashMap::new())
        }
    }
}
