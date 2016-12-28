use lime::envelope::*;
use lime::ErrReason;
use lime::JsonMap;

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

// Thanks to github user 'dtolnay' for help with the following code...
use serde::ser::{Serialize, Serializer};
use lime::helper::NotificationEventHelper;

impl Serialize for Notification {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        /// Private helper that reflects the structure of the output JSON.
        #[derive(Serialize)]
        struct NotificationHelper<'a> {
            // TODO: Get 'Node' method to get str from 'Node'
            to: Option<&'a str>,
            from: Option<&'a str>,
            pp: Option<&'a str>,
            id: &'a str,
            metadata: Option<&'a JsonMap>,

            event: NotificationEventHelper,
            #[serde(skip_serializing_if = "Option::is_none")]
            reason: Option<&'a ErrReason>,
        }

        use lime::helper::NotificationEventHelper::*;

        let (event, reason) = match self.event {
            NotificationEvent::Accepted     => (Accepted, None),
            NotificationEvent::Received     => (Received, None),
            NotificationEvent::Validated    => (Validated, None),
            NotificationEvent::Authorized   => (Authorized, None),
            NotificationEvent::Dispatched   => (Dispatched, None),
            NotificationEvent::Consumed     => (Consumed, None),
            NotificationEvent::Failed(ref reason) => (Failed, Some(reason)),
        };

        NotificationHelper {
            to: self.to.as_ref().map(|s| &**s),
            from: self.from.as_ref().map(|s| &**s),
            pp: self.pp.as_ref().map(|s| &**s),
            id: &self.id,
            metadata: self.metadata.as_ref(),

            event: event,
            reason: reason,
        }.serialize(serializer)
    }
}
