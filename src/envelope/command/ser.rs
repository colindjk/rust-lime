use serde::ser::{Serialize, Serializer};
use envelope::{JsonMap, ErrReason, MsgID};
use envelope::helper::CommandStatusHelper;
use envelope::command::*;

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
            id: Option<&'a MsgID>,
            metadata: Option<&'a JsonMap>,

            method: &'a CommandMethod,
            status: Option<&'a CommandStatusHelper>,
            reason: Option<&'a ErrReason>,

            uri: Option<&'a str>,
            #[serde(rename="type")]
            mime_type: Option<&'a str>,
        }

        use envelope::helper::CommandStatusHelper::*;

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
            id: self.id.as_ref(),
            metadata: self.metadata.as_ref(),

            method: &self.method,
            status: status.as_ref(),
            reason: reason,

            uri: self.uri.as_ref().map(|s| &**s),
            mime_type: self.mime_type.as_ref().map(|s| &**s),
        }.serialize(serializer)
    }
}

