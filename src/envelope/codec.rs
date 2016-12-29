use tokio_core::io::{
    Codec,
    EasyBuf
};

use std::io;
use std::str;
use envelope::{ SealedEnvelope, DELIMITER };

pub struct LimeCodec;

impl Codec for LimeCodec {
    type In = (u64, SealedEnvelope);
    type Out = (u64, SealedEnvelope);

    fn decode(&mut self, buf: &mut EasyBuf)
              -> Result<Option<Self::In>, io::Error> {
        match buf.as_slice().iter().position(|&b| b == DELIMITER) {
            Some(index) => {
                let buf = buf.drain_to(index + 1);
                use serde_json::from_slice;
                if let Ok(e) = from_slice::<SealedEnvelope>(buf.as_slice()) {
                    Ok(Some((e.id().unwrap_or(0), e)))
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
        if let Ok(mut json) = to_vec(&msg.1) {
            buf.append(&mut json);
            buf.push(DELIMITER.clone());
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other, "Failed to encode object".to_string()))
        }
    }

}

