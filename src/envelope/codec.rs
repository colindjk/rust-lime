use tokio_core::io::{
    Codec,
    Framed,
    EasyBuf
};

use std::io;
use std::str;
use envelope::{ Envelope, DELIMITER };

/// Type used by the Node structs for input and output, allowing a user to
/// set up a Node which communicates over any type of network connection.
pub type EnvelopeStream<T> = Framed<T, LimeCodec>;
pub struct LimeCodec;

impl Codec for LimeCodec {
    type In = Envelope;
    type Out = Envelope;

    fn decode(&mut self, buf: &mut EasyBuf)
            -> Result<Option<Self::In>, io::Error> {
        match buf.as_slice().iter().position(|&b| b == DELIMITER) {
            Some(index) => {
                let buf = buf.drain_to(index + 1);
                use serde_json::from_slice;
                if let Ok(e) = from_slice::<Envelope>(buf.as_slice()) {
                    Ok(Some(e))
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Failed to decode object".to_string()))
                }
            }
            None => Ok(None)
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        use serde_json::ser::to_vec;
        if let Ok(mut json) = to_vec(&msg) {
            buf.append(&mut json);
            buf.push(DELIMITER.clone());
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other, "Failed to encode object".to_string()))
        }
    }

}

pub struct LimeMultiCodec;

impl Codec for LimeMultiCodec {
    type In = (u64, Envelope);
    type Out = (u64, Envelope);

    fn decode(&mut self, buf: &mut EasyBuf)
            -> Result<Option<Self::In>, io::Error> {
        match LimeCodec.decode(buf) {
            Ok(Some(envelope)) =>
                Ok(Some((envelope.id().unwrap_or(0), envelope))),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        let (_, msg) = msg;
        LimeCodec.encode(msg, buf)
    }

}
