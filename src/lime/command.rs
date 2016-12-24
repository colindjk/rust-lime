use serde_json::{ Map, Value };

use lime::envelope::*;

pub struct Command {
    to: Option<Node>,
    from: Option<Node>,
    pp: Option<Node>,
    id: MsgID,
    metadata: Option<Value>,

    method: CommandMethod,
    uri: Option<String>,
    #[serde(rename="type")]
    mime_type: Option<String>,

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

