use serde_json::{ Map, Value };

use lime::envelope::*;
use lime::JsonMap;

// TODO: How to parse the session? seems real complicated currently. 

/// Sent by server, contains options for authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRequest {
    pub to: Option<Node>,
    pub from: Option<Node>, // mandatory for clients during auth
    pub pp: Option<Node>,
    pub id: MsgID,
    pub metadata: Option<JsonMap>,

    pub state: SessionState,

    #[serde(rename="encryptionOptions")]
    pub encryption_options: Option<Vec<String>>,
    #[serde(rename="compressionOptions")]
    pub compression_options: Option<Vec<String>>,
    #[serde(rename="schemeOptions")]
    pub scheme_options: Option<Vec<Value>>,
}

/// Sent by server, contains options for authentication
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionState {
    #[serde(rename="new")]              New,
    #[serde(rename="negotiating")]      Negotiating,
    #[serde(rename="authenticating")]   Authenticating,
    #[serde(rename="established")]      Established,
    #[serde(rename="finishing")]        Finishing,
    #[serde(rename="finished")]         Finished,
    #[serde(rename="failed")]           Failed,
}

