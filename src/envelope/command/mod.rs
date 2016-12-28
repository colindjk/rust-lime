use envelope::{ErrReason, JsonMap, Node, MsgID};

mod ser;

#[derive(Debug)]
pub struct Command {
    pub to: Option<Node>,
    pub from: Option<Node>,
    pub pp: Option<Node>,
    pub id: Option<MsgID>,
    pub metadata: Option<JsonMap>,

    pub method: CommandMethod,
    pub status: Option<CommandStatus>,

    pub uri: Option<String>,
    pub mime_type: Option<String>,
}

/// Signifies the event which pertains to a previously dealt with message.
/// Uses 'id' from sent message to determine which one should happen.
/// TODO: Unique set of 'id's per user or nah?
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CommandMethod {
    #[serde(rename="get")]          Get,
    #[serde(rename="set")]          Set,
    #[serde(rename="delete")]       Delete,
    #[serde(rename="subscribe")]    Subscribe,
    #[serde(rename="unsubscribe")]  Unsubscribe,
    #[serde(rename="observe")]      Observe,
}

#[derive(Debug, PartialEq)]
pub enum CommandStatus {
    Success,
    Failure(ErrReason),
}

