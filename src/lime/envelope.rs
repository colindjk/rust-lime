/// This section includes all info pertaining to parsing the LIME protocol
/// into messages.

/// This will be split into many files when I get the understanding needed.

// TODO: Create a client connection struct which will handle default values for
// protocol fields.

//use tokio::io::{
    //Codec, EasyBuf
//};

//use json::ser::{to_vec};
//use json::de::{from_slice};

use std::collections::{ HashMap };

pub type UserID = String;
pub type MsgID = String;
pub type TimeStamp = u64;

/// Both 'from' and 'to', fields are optional, if not given, they will revert
/// to default values (UserID of node who sent the message). They remain
/// optional because the sender doesn't need to fill them out.
#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope {
    id: String,
    from: Option<UserID>,
    to: Option<UserID>,
    pp: Option<UserID>,
    // TODO: figure out how to incorporate metadata as a type
    metadata: HashMap<String, String>,
    kind: EnvelopeKind,
}

/// Outlines the kinds of envelopes one can receive.
/// TODO: Figure out if values as '&str' efficient / possible / worth.
#[derive(Debug, Serialize, Deserialize)]
pub enum EnvelopeKind {
    Message {
        mime_type: String,
        content: String,
    },
    Notification {
        event: NotificationEvent,
    },
    Command {
        mime_type: String,
        method: CommandMethod,
        uri: String,

    },
    Session {

    }
}

/// Signifies the event which pertains to a previously dealt with message.
/// Uses 'id' from sent message to determine which one should happen.
/// TODO: Unique set of 'id's per user or nah?
#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationEvent {
    #[serde(rename="accepted")]     Accepted,
    #[serde(rename="validated")]    Validated,
    #[serde(rename="authorized")]   Authorized,
    #[serde(rename="dispatched")]   Dispatched,
    #[serde(rename="received")]     Received,
    #[serde(rename="consumed")]     Consumed,
    #[serde(rename="failed")]       Failed(Reason),
}

/// Signifies the event which pertains to a previously dealt with message.
/// Uses 'id' from sent message to determine which one should happen.
/// TODO: Unique set of 'id's per user or nah?
#[derive(Debug, Serialize, Deserialize)]
pub enum CommandMethod {
    #[serde(rename="get")]          Get,
    #[serde(rename="set")]          Set,
    #[serde(rename="delete")]       Delete,
    #[serde(rename="subscribe")]    Subscribe,
    #[serde(rename="unsubscribe")]  Unsubscribe,
    #[serde(rename="observe")]      Observe,
}

/// When an Error occurs, this will exist.
#[derive(Debug, Serialize, Deserialize)]
pub struct Reason {
    code: u8,
    description: String
}

