use futures::Future;
use tokio::reactor;
use tokio_service::Service;

use message::{ Request, Response, Error };

/// Server which will exist for every interaction...?
pub struct Server;

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        unimplemented!();
    }
}
