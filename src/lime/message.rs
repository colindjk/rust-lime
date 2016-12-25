use serde_json::{ Map, Value };

use lime::envelope::*;
use lime::JsonMap;

pub type Content = Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    to: Option<Node>,
    from: Option<Node>,
    pp: Option<Node>,
    id: Option<MsgID>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    metadata: JsonMap,

    #[serde(rename="type")]
    mime_type: String,
    content: Content,
}

impl_Envelope!(Message,
               MessageType,
               |_| Some(MessageType::Normal),
               Some(MessageType::Normal),
               "content");

/// TODO: Figure out other possible message types.
pub enum MessageType {
    Normal,
    Chat,
    Groupchat,
    Error
}

