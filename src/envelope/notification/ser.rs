// Thanks to github user 'dtolnay' for help with the following code...
use serde::ser::{Serialize, Serializer};
use envelope::{JsonMap, ErrReason, MsgID};
use envelope::helper::NotificationEventHelper;
use envelope::notification::*;

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
            id: &'a MsgID,
            metadata: Option<&'a JsonMap>,

            event: NotificationEventHelper,
            reason: Option<&'a ErrReason>,
        }

        use envelope::helper::NotificationEventHelper::*;

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
