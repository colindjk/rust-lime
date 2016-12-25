use tokio::io::{
    Codec,
    EasyBuf
};

use std::io;
use std::str;
use lime::{ SealedEnvelope, DELIMITER };
use serde_json::value::*;

pub struct LimeCodec;

impl Codec for LimeCodec {
    type In = SealedEnvelope;
    type Out = SealedEnvelope;

    fn decode(&mut self, buf: &mut EasyBuf)
              -> Result<Option<Self::In>, io::Error> {
        let index = match buf.as_slice().iter().position(|&b| b == DELIMITER) {
            Some(index) => {
                println!("Decoding {}", index);
                index
            }
            None => return Ok(None)
        };
        let buf = buf.drain_to(index + 1);
        let buf = str::from_utf8(buf.as_slice()).unwrap();
    
        use serde_json::from_str;
        use lime::SealedEnvelope::*;

        let env = if buf.contains("content") {
            Message(from_str(buf).unwrap())
        }
        else if buf.contains("event") {
            Notification(from_str(buf).unwrap())
        }
        else if buf.contains("method") {
            Command(from_str(buf).unwrap())
        }
        else if buf.contains("state") {
            if buf.contains("encryption")  ||
               buf.contains("compression") || 
               buf.contains("compression") 
            {
                SessionRes(from_str(buf).unwrap())
            } else {
                SessionReq(from_str(buf).unwrap())
            }
        }
        else {
            Other(from_str(buf).unwrap())
        };
        Ok(Some(env))
    }

    // TODO: Make a Envelope based method which produces a Vec<u8>, or
    // implement serialize for SealedEnvelope.
    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        panic!()
    }
}

