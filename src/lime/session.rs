use serde_json::{ Value };

use lime::envelope::*;
use lime::JsonMap;

// TODO: How to parse the session? seems real complicated currently. 

/// Sent by server, contains options for authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRequest {
    to: Option<Node>,
    from: Option<Node>, // mandatory for clients during auth
    pp: Option<Node>,
    id: MsgID,
    metadata: Option<JsonMap>,

    state: SessionState,

    #[serde(rename="encryptionOptions")]
    encryption_options: Option<Vec<String>>,
    #[serde(rename="compressionOptions")]
    compression_options: Option<Vec<String>>,
    #[serde(rename="schemeOptions")]
    scheme_options: Option<Vec<Value>>,
}

/// Sent by server, contains options for authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    to: Option<Node>,
    from: Option<Node>, // mandatory for clients during auth
    pp: Option<Node>,
    id: MsgID,
    metadata: Option<JsonMap>,

    state: SessionState,

    encryption: Option<String>,
    compression: Option<String>,
    scheme: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
enum SessionState {
    #[serde(rename="new")]              New,
    #[serde(rename="negotiating")]      Negotiating,
    #[serde(rename="authenticating")]   Authenticating,
    #[serde(rename="established")]      Established,
    #[serde(rename="finishing")]        Finishing,
    #[serde(rename="finished")]         Finished,
    #[serde(rename="failed")]           Failed,
}

