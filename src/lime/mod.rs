use serde_json::{ Map, Value };

/// -- Global Constants --
pub static DELIMITER : u8 = b'\n' as u8;
type JsonMap = Map<String, Value>;

#[macro_use]
pub mod envelope;
pub mod message;
pub mod notification;
pub mod command;
pub mod session;

use self::envelope::Envelope;
use self::message::Message;
use self::notification::Notification;
use self::command::Command;
use self::session::*;

/// Outlines the kinds of envelopes one can receive.
/// TODO: Figure out if values as '&str' efficient / possible / worth.
#[derive(Debug)]
pub struct SealedEnvelope<T: Envelope>(T);

///// Deserialization implementation distinguishes the specific type of 'frame'
///// being received.
//impl Deserialize for EnvelopeKind {
//    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
//        where D: Deserializer,
//    {
//    }
//}

