//use std::io;
//use tokio_service::Service;
//use futures::future;

//use lime::envelope::{ Envelope };

//#[allow(dead_code)]
//pub struct ClientConnection;

///// The server service. Yeah...
///// TODO: Figure out way to differentiate between Envelopes and Envelope
///// errors.
//#[allow(dead_code)]
//impl Service for ClientConnection {
    //type Request = Envelope;
    //type Response = Envelopee;
    //type Error = io::Error;
    //type Future = Box<future::Future<
        //Item = Self::Response,
        //Error = Self::Error>>;

    //#[allow(unused_variables)]
    //fn call(&self, req: Self::Request) -> Self::Future {
        //unimplemented!();
    //}
//}
