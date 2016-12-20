use tokio::io::{
    Codec,
    EasyBuf
};

use lime::envelope;

pub struct LimeCodec;

/// Below will be composed of elements defined in the above LIME protocol
/// section.
pub enum Request {

}

pub enum Response {

}

pub struct Error {
    reason: envelope::Reason,
}


