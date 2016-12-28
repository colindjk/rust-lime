use serde_json::{ Map, Value };

/// -- Global Constants --
pub static DELIMITER : u8 = b'\n' as u8;
type JsonMap = Map<String, Value>;

mod ser;

pub mod message;
pub mod notification;
pub mod command;
pub mod session;

mod codec;
mod helper;

pub use self::codec::LimeCodec;
pub use self::message::{Message, Content};
pub use self::notification::{Notification, NotificationEvent};
pub use self::command::Command;
pub use self::session::*;

pub type Node = String;
pub type MsgID = String;
pub type TimeStamp = u64;

/// Outlines the kinds of envelopes one can receive.
/// TODO: Resource field as separate struct, uri?
#[derive(Debug)]
pub enum SealedEnvelope {
    Message(Message),
    Notification(Notification),
    Command(Command),
    SessionReq(SessionRequest),
    SessionRes(SessionResponse),
    Unknown(JsonMap),
}

/// When an Error occurs, this will exist.
/// TODO: Use this for other structs aside from just Notification.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrReason {
    pub code: u8,
    pub description: Option<String>
}

/// Trait for all envelope related types.
/// TODO: Include 'pp' and 'metadata'
/// TODO: Convert to MIME
pub trait Envelope {
    //type Ty;

    //fn unique<'a>() -> &'a str;

    fn id(&self) -> Option<&str>;
    fn to(&self) -> Option<&str>;
    fn from(&self) -> Option<&str>;
    //fn pp(&self) -> Option<Node>;
    //fn metadata(&self) -> Option<Node>;

    fn set_id(&mut self, Option<String>);
    fn set_to(&mut self, Option<String>);
    fn set_from(&mut self, Option<String>);
    //fn set_pp(&self) -> Option<Node>;
    //fn set_metadata(&self) -> Option<Node>;
}

macro_rules! impl_Envelope(
    ($kind: ident, $ty: ty, $ty_some: expr, $ty_none: expr, $unique_field: expr) => (
        impl Envelope for $kind {
            //type Ty = $ty;

            //fn unique<'a>() -> &'a str { $unique_field }

            fn id(&self) -> Option<&str> { self.id.as_ref().map(|s| &**s) }
            fn to(&self) -> Option<&str> { self.to.as_ref().map(|s| &**s) }
            fn from(&self) -> Option<&str> { self.from.as_ref().map(|s| &**s) }
            fn set_id(&mut self, id: Option<String>) { self.id = id; }
            fn set_to(&mut self, to: Option<String>) { self.to = to; }
            fn set_from(&mut self, from: Option<String>) { self.from = from; }

        }

    );
);

