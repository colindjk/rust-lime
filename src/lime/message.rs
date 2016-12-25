use serde_json::{ Value };

use lime::envelope::*;
use lime::JsonMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    map: JsonMap,

    to: Option<Node>,
    from: Option<Node>,
    pp: Option<Node>,
    id: Option<MsgID>,
    metadata: Option<JsonMap>,

    #[serde(rename="type")]
    mime_type: String,
    content: String,
}

impl_Envelope!(Message,
               MessageType,
               |_| Some(MessageType::Normal),
               Some(MessageType::Normal));

/// TODO: Figure out other possible message types.
pub enum MessageType {
    Normal,
    Chat,
    Groupchat,
    Error
}

