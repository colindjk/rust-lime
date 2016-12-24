
/// -- Global Constants --
pub static DELIMITER : u8 = b'\n' as u8;

#[macro_use]
pub mod envelope;
pub mod message;
pub mod notification;
pub mod command;
pub mod session;

use self::message::Message;
use self::notification::Notification;
use self::command::Command;
use self::session::*;

/// Outlines the kinds of envelopes one can receive.
/// TODO: Figure out if values as '&str' efficient / possible / worth.
//#[derive(Debug, Serialize, Deserialize)]
pub enum EnvelopeKind {
    Message(Message),
    Notification(Notification),
    Command(Command),
    SessionRequest(SessionRequest),
    SessionResponse(SessionResponse)
}

