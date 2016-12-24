use serde_json::{ Map, Value };

pub struct Session {
    map: Map<String, Value>,

    to: Option<Node>,
    from: Option<Node>, // mandatory for clients during auth
    pp: Option<Node>,
    id: MsgID,
    metadata: Option<Value>,

    state: SessionState,
    negotiation: Option<Negotiation>, // present during... negotiation
}

enum Negotiation {
    Request(SessionOptions),
    Response(SessionResponse),
}

struct SessionOptions {
    encryption: Vec<String>,
    compression: Vec<String>,
    scheme: Vec<Value>,
}

struct SessionResponse {
    encryption: Option<String>,
    compression: Option<String>,
    scheme: Option<Value>,
}

enum SessionState {
    #[serde(rename="new")]              New,
    #[serde(rename="negotiating")]      Negotiating,
    #[serde(rename="authenticating")]   Authenticating,
    #[serde(rename="established")]      Established,
    #[serde(rename="finishing")]        Finishing,
    #[serde(rename="finished")]         Finished,
    #[serde(rename="failed")]           Failed,
}

