use std::io;

use futures::{future, Future, BoxFuture};
use tokio_core::io::{Io, Framed};
use tokio_service::Service;
use tokio_proto::multiplex::ServerProto;

use envelope::{LimeCodec, SealedEnvelope as Envelope};

pub struct LimeProtocol;

/// TODO: Make sure the Request gets to the right place (user / db?), then start
/// to worry about the actual response (what the future returns).

/// This, uh... 'hooks up the codec'. Yeah.
impl<T: Io + 'static> ServerProto<T> for LimeProtocol {
    type Request = Envelope;
    type Response = Envelope;
    type Error = io::Error;
    type Transport = Framed<T, LimeCodec>;
    type BindTransport = io::Result<Framed<T, LimeCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, LimeCodec>> {
        Ok(io.framed(LimeCodec))
    }
}

pub struct Server;

impl Service for Server {
    type Request = Envelope;
    type Response = Envelope;
    type Error = io::Error;
    type Future = BoxFuture<Envelope, io::Error>;

    /// For now we'll just return the message that was sent
    fn call(&self, req: Envelope) -> Self::Future {
        future::finished(req).boxed()
    }
}

