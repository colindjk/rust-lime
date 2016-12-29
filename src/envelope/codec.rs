use tokio_core::io::{
    Codec,
    EasyBuf
};

use std::io;
use std::str;
use envelope::{ SealedEnvelope, DELIMITER };

pub struct LimeCodec;

impl Codec for LimeCodec {
    type In = SealedEnvelope;
    type Out = SealedEnvelope;

    fn decode(&mut self, buf: &mut EasyBuf)
              -> Result<Option<Self::In>, io::Error> {
        match buf.as_slice().iter().position(|&b| b == DELIMITER) {
            Some(index) => {
                let buf = buf.drain_to(index + 1);
                use serde_json::from_slice;
                if let Ok(envelope) = from_slice(buf.as_slice()) {
                    Ok(Some(envelope))
                } else {
                    unimplemented!()
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

