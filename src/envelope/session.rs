use serde_json::{ Value };

use lime::envelope::*;
use lime::{JsonMap, ErrReason};

// TODO: How to parse the session? seems real complicated currently. 

/// Sent by server, contains options for authentication
#[derive(Debug)]
pub struct SessionRequest {
    pub to: Option<Node>,
    pub from: Option<Node>, // mandatory for clients during auth
    pub pp: Option<Node>,
    pub id: MsgID,
    pub metadata: Option<JsonMap>,

    pub state: SessionState,

    pub encryption_options: Option<Vec<String>>,
    pub compression_options: Option<Vec<String>>,
    pub scheme_options: Option<Vec<Value>>,
}

use serde::ser::{Serialize, Serializer};
use lime::helper::SessionStateHelper;

impl Serialize for SessionRequest {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        /// Private helper that reflects the structure of the output JSON.
        #[derive(Serialize)]
        struct SessionRequestHelper<'a> {
            // TODO: Get 'Node' method to get str from 'Node'
            to: Option<&'a str>,
            from: Option<&'a str>,
            pp: Option<&'a str>,
            id: &'a str,
            metadata: Option<&'a JsonMap>,

            state: &'a SessionStateHelper,

            reason: Option<&'a ErrReason>,
            #[serde(rename="encryptionOptions")]
            encryption_options: Option<&'a Vec<String>>,
            #[serde(rename="compressionOptions")]
            compression_options: Option<&'a Vec<String>>,
            #[serde(rename="schemeOptions")]
            scheme_options: Option<&'a Vec<Value>>,
        }

        use lime::helper::SessionStateHelper::*;

        let (state, reason) = match self.state {
            SessionState::New            => (New, None),
            SessionState::Negotiating    => (Negotiating, None),
            SessionState::Authenticating => (Authenticating, None),
            SessionState::Established    => (Established, None),
            SessionState::Finishing      => (Finishing, None),
            SessionState::Finished       => (Finished, None),
            SessionState::Failed(ref r)  => (Failed, Some(r)),
        };

        SessionRequestHelper {
            to: self.to.as_ref().map(|s| &**s),
            from: self.from.as_ref().map(|s| &**s),
            pp: self.pp.as_ref().map(|s| &**s),
            id: &self.id,
            metadata: self.metadata.as_ref(),

            state: &state,

            reason: reason,
            encryption_options: self.encryption_options.as_ref(),
            compression_options: self.compression_options.as_ref(),
            scheme_options: self.scheme_options.as_ref(),
        }.serialize(serializer)
    }
}

/// Sent by server, contains options for authentication
#[derive(Debug)]
pub struct SessionResponse {
    pub to: Option<Node>,
    pub from: Option<Node>, // mandatory for clients during auth
    pub pp: Option<Node>,
    pub id: MsgID,
    pub metadata: Option<JsonMap>,

    pub state: SessionState,

    pub encryption: Option<String>,
    pub compression: Option<String>,
    pub scheme: Option<Value>,
}

impl Serialize for SessionResponse {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        /// Private helper that reflects the structure of the output JSON.
        #[derive(Serialize)]
        struct SessionResponseHelper<'a> {
            // TODO: Get 'Node' method to get str from 'Node'
            to: Option<&'a str>,
            from: Option<&'a str>,
            pp: Option<&'a str>,
            id: &'a str,
            metadata: Option<&'a JsonMap>,

            state: &'a SessionStateHelper,

            reason: Option<&'a ErrReason>,
            encryption: Option<&'a str>,
            compression: Option<&'a str>,
            scheme: Option<&'a Value>,
        }

        use lime::helper::SessionStateHelper::*;

        let (state, reason) = match self.state {
            SessionState::New            => (New, None),
            SessionState::Negotiating    => (Negotiating, None),
            SessionState::Authenticating => (Authenticating, None),
            SessionState::Established    => (Established, None),
            SessionState::Finishing      => (Finishing, None),
            SessionState::Finished       => (Finished, None),
            SessionState::Failed(ref r)  => (Failed, Some(r)),
        };

        SessionResponseHelper {
            to: self.to.as_ref().map(|s| &**s),
            from: self.from.as_ref().map(|s| &**s),
            pp: self.pp.as_ref().map(|s| &**s),
            id: &self.id,
            metadata: self.metadata.as_ref(),

            state: &state,

            reason: reason,
            encryption: self.encryption.as_ref().map(|s| &**s),
            compression: self.compression.as_ref().map(|s| &**s),
            scheme: self.scheme.as_ref(),
        }.serialize(serializer)
    }
}

#[derive(Debug, PartialEq)]
pub enum SessionState {
    New,
    Negotiating,
    Authenticating,
    Established,
    Finishing,
    Finished,
    Failed(ErrReason),
}

