use futures::Future;
use tokio::reactor;
use tokio_service::Service;

use frames::message::{ Request, Response, Error };

/// Server which will exist for every interaction...?
pub struct Server;

