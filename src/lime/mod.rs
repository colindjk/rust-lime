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
pub mod codec;

pub use self::envelope::Envelope;
pub use self::message::{Message, Content};
pub use self::notification::Notification;
pub use self::command::Command;
pub use self::session::*;

/// Outlines the kinds of envelopes one can receive.
/// TODO: HOW SHOULD I HANDLE DIFFERENT ENVELOPE TYPES WAAA
#[derive(Debug)]
pub enum SealedEnvelope {
    Message(Message),
    Notification(Notification),
    Command(Command),
    SessionReq(SessionRequest),
    SessionRes(SessionResponse),
    Other(JsonMap),
}

use serde::{Serialize, Serializer, Deserialize, Deserializer};

/// Deserialization implementation distinguishes the specific type of 'frame'
/// being received.
impl Deserialize for SealedEnvelope {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer,
    {
        let helper : Value = Deserialize::deserialize(deserializer)?;

        use serde_json::from_value;
        use self::SealedEnvelope::*;

        let env = if helper.find("content") != None {
            Message(from_value(helper).unwrap())
        }
        else if helper.find("event") != None {
            Notification(from_value(helper).unwrap())
        }
        else if helper.find("method") != None {
            Command(from_value(helper).unwrap())
        }
        else if helper.find("state") != None {
            if helper.find("encryption")  != None ||
               helper.find("compression") != None || 
               helper.find("compression") != None
            {
                SessionRes(from_value(helper).unwrap())
            } else {
                SessionReq(from_value(helper).unwrap())
            }
        }
        else {
            Other(from_value(helper).unwrap())
        };

        Ok(env)
    }
}

