use serde_json::{ Map, Value };

pub struct Message { map: Map<String, Value> }

/// TODO: Figure out other possible message types.
pub enum MessageType {
    Chat,
    Groupchat,
    Error
}

impl_Envelope!(Message, );

