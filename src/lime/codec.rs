use tokio::io::{
    Codec,
    EasyBuf
};

use std::io;
use lime::{ SealedEnvelope, DELIMITER };
use serde_json::value::*;
use serde_json;

pub struct LimeCodec;

impl Codec for LimeCodec {
    type In = SealedEnvelope;
    type Out = SealedEnvelope;

    fn decode(&mut self, buf: &mut EasyBuf)
              -> Result<Option<Self::In>, io::Error> {
        match buf.as_slice().iter().position(|&b| b == DELIMITER) {
            Some(index) => {
                println!("Decoding {}", index);
                let object_buf = buf.drain_to(index + 1);
                // TODO: Clean the below up so it doesn't blow up
                Ok(Some(serde_json::from_slice(object_buf.as_slice()).unwrap()))
            }
            None => Ok(None)
        }
    }

    // TODO: Make a Envelope based method which produces a Vec<u8>, or
    // implement serialize for SealedEnvelope.
    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        panic!()
    }
}

