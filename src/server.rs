//use futures::Future;
//use tokio::reactor;
//use tokio_service::Service;

//use std::net::SocketAddr;
//use std::collections::HashMap;
//use std::sync::Arc;

//use futures::sync::mpsc::{
    //UnboundedSender as Sender,
    //UnboundedReceiver as Receiver,
    //unbounded as channel
//};

//use tokio_core::io::Io;
//use tokio_core::net::{TcpListener};
//use tokio_core::reactor::{Core};

//use lime::*;

///// Server which will exist for every interaction...?
//pub struct Server {
    //core: Core,
    //addr: SocketAddr,

    //clients: Arc<HashMap<Node, Sender<SealedEnvelope>>>,
    //#[allow(dead_code)]
    //groups: Arc<HashMap<Node, Sender<SealedEnvelope>>>,
    //channel: (Sender<SealedEnvelope>, Sender<SealedEnvelope>),
//}

