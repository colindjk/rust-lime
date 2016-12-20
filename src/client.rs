use tokio_service::Service;
use futures::future;

use lime::message::{ Request, Response, Error };

#[allow(dead_code)]
pub struct ClientConnection;

/// The server service. Yeah...
#[allow(dead_code)]
impl Service for ClientConnection {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<future::Future<Item = Self::Response, Error = Self::Error>>;

    #[allow(unused_variables)]
    fn call(&self, req: Self::Request) -> Self::Future {
        unimplemented!();
    }
}
