use serde_json::{ Map, Value };

use lime::envelope::*;

pub struct Message {
    map: Map<String, Value>,

    to: Option<Node>,
    from: Option<Node>,
    pp: Option<Node>,
    id: Option<MsgID>,
    metadata: Option<Value>,

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

