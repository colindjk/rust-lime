use serde_json::{ Map, Value };
use lime::envelope::*;

pub struct Message { map: Map<String, Value> }

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

