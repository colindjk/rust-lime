use tokio_service::Service;

pub struct ClientConnection;

/// The server service. Yeah...
impl Service for ClientConnection {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        unimplemented!();
    }
}
