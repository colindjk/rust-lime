use lime::envelope::*;
use lime::{ErrReason, JsonMap};

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

use serde::ser::{Serialize, Serializer};
use lime::helper::CommandStatusHelper;

impl Serialize for Command {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        /// Private helper that reflects the structure of the output JSON.
        #[derive(Serialize)]
        struct CommandHelper<'a> {
            // TODO: Get 'Node' method to get str from 'Node'
            to: Option<&'a str>,
            from: Option<&'a str>,
            pp: Option<&'a str>,
            id: Option<&'a str>,
            metadata: Option<&'a JsonMap>,

            method: &'a CommandMethod,
            status: Option<&'a CommandStatusHelper>,
            reason: Option<&'a ErrReason>,

            uri: Option<&'a str>,
            #[serde(rename="type")]
            mime_type: Option<&'a str>,
        }

        use lime::helper::CommandStatusHelper::*;

        let (status, reason) = match self.status {
            Some(CommandStatus::Success)
                => (Some(Success), None),
            Some(CommandStatus::Failure(ref reason))
                => (Some(Failure), Some(reason)),
            None => (None, None)
        };

        CommandHelper {
            to: self.to.as_ref().map(|s| &**s),
            from: self.from.as_ref().map(|s| &**s),
            pp: self.pp.as_ref().map(|s| &**s),
            id: self.id.as_ref().map(|s| &**s),
            metadata: self.metadata.as_ref(),

            method: &self.method,
            status: status.as_ref(),
            reason: reason,

            uri: self.uri.as_ref().map(|s| &**s),
            mime_type: self.mime_type.as_ref().map(|s| &**s),
        }.serialize(serializer)
    }
}
