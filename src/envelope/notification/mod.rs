use envelope::envelope::*;
use envelope::ErrReason;
use envelope::JsonMap;

mod ser;

#[derive(Debug)]
pub struct Notification {
    pub to: Option<Node>,
    pub from: Option<Node>,
    pub pp: Option<Node>,
    pub id: MsgID,
    pub metadata: Option<JsonMap>,

    pub event: NotificationEvent,
}

/// Signifies the event which pertains to a previously dealt with message.
/// Uses 'id' from sent message to determine which one should happen.
/// TODO: Unique set of 'id's per user or nah?
#[derive(Debug, PartialEq)]
pub enum NotificationEvent {
    Accepted,
    Validated,
    Authorized,
    Dispatched,
    Received,
    Consumed,
    Failed(ErrReason),
}

