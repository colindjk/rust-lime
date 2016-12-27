//use serde_json::Map;

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
#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationEvent {
    #[serde(rename="accepted")]     Accepted,
    #[serde(rename="validated")]    Validated,
    #[serde(rename="authorized")]   Authorized,
    #[serde(rename="dispatched")]   Dispatched,
    #[serde(rename="received")]     Received,
    #[serde(rename="consumed")]     Consumed,
    #[serde(rename="failed")]       Failed(ErrReason),
}

// Thanks to github user 'dtolnay' for help with the following code...

//use serde::{Serialize, Serializer, Deserialize, Deserializer};

//impl Deserialize for Notification {
    //fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        //where D: Deserializer
    //{
        ///// Private helper that reflects the structure of the input JSON.
        //#[derive(Deserialize)]
        //struct NotificationHelper {
            //to: Option<Node>,
            //from: Option<Node>,
            //pp: Option<Node>,
            //id: MsgID,
            //metadata: Option<JsonMap>,

            //event: NotificationEventHelper,
            //reason: Option<Reason>,
        //}

        //use self::NotificationEventHelper::*;
        //use serde::de::Error;

        //let helper: NotificationHelper =
            //Deserialize::deserialize(deserializer)?;

        //let event = match (helper.event, helper.reason) {
            //(Accepted, None)       => NotificationEvent::Accepted,
            //(Received, None)       => NotificationEvent::Received,
            //(Validated, None)      => NotificationEvent::Validated,
            //(Authorized, None)     => NotificationEvent::Authorized,
            //(Dispatched, None)     => NotificationEvent::Dispatched,
            //(Consumed, None)       => NotificationEvent::Consumed,
            //(Failed, Some(reason)) => NotificationEvent::Failed(reason),

            //(_, Some(_))   => return Err(Error::custom("unexpected reason")),
            //(Failed, None) => return Err(Error::custom("missing reason")),
        //};

        //Ok(Notification {
            //to: helper.to,
            //from: helper.from,
            //pp: helper.pp,
            //id: helper.id,
            //metadata: helper.metadata,

            //event: event,
        //})
    //}
//}

//impl Serialize for Notification {
    //fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        //where S: Serializer
    //{
        ///// Private helper that reflects the structure of the output JSON.
        //#[derive(Serialize)]
        //struct NotificationHelper<'a> {
            //// TODO: Get 'Node' method to get str from 'Node'
            //to: Option<&'a str>,
            //from: Option<&'a str>,
            //pp: Option<&'a str>,
            //id: &'a str,
            //metadata: &'a JsonMap,

            //event: NotificationEventHelper,
            //#[serde(skip_serializing_if = "Option::is_none")]
            //reason: Option<&'a Reason>,
        //}

        //use self::NotificationEventHelper::*;

        //let (event, reason) = match self.event {
            //NotificationEvent::Accepted     => (Accepted, None),
            //NotificationEvent::Received     => (Received, None),
            //NotificationEvent::Validated    => (Validated, None),
            //NotificationEvent::Authorized   => (Authorized, None),
            //NotificationEvent::Dispatched   => (Dispatched, None),
            //NotificationEvent::Consumed     => (Consumed, None),
            //NotificationEvent::Failed(ref reason) => (Failed, Some(reason)),
        //};

        //NotificationHelper {
            //to: self.to.as_ref().map(|s| &**s),
            //from: self.from.as_ref().map(|s| &**s),
            //pp: self.pp.as_ref().map(|s| &**s),
            //id: &self.id,
            //metadata: &self.metadata,

            //event: event,
            //reason: reason,
        //}.serialize(serializer)
    //}
//}
