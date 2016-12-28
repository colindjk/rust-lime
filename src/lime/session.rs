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

#[derive(Debug)]
pub enum SessionState {
    New,
    Negotiating,
    Authenticating,
    Established,
    Finishing,
    Finished,
    Failed(ErrReason),
}

