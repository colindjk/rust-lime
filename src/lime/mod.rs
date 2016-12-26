use serde_json::{ Map, Value };

/// -- Global Constants --
pub static DELIMITER : u8 = b'\n' as u8;
type JsonMap = Map<String, Value>;

#[macro_use]
pub mod envelope;
pub mod message;
pub mod notification;
pub mod command;
pub mod session;
pub mod codec;

pub use self::codec::LimeCodec;
pub use self::envelope::*;
pub use self::message::{Message, Content};
pub use self::notification::Notification;
pub use self::command::Command;
pub use self::session::*;

/// Outlines the kinds of envelopes one can receive.
/// TODO: HOW SHOULD I HANDLE DIFFERENT ENVELOPE TYPES WAAA
#[derive(Debug)]
pub enum SealedEnvelope {
    Message(Message),
    Notification(Notification),
    Command(Command),
    SessionReq(SessionRequest),
    SessionRes(SessionResponse),
    Unknown(JsonMap),
}

// SerDe section

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, MapVisitor, Error as DeError};

/// Contains all known fields an Envelope can contain.
enum FieldHelper {
    To, From, Pp, Id, Metadata, // Common fields
    // Unique fields
    Content, // Message
    Event,   // Notification
    Method,  // Command
    State,   // Session[Request|Response]
    // Extra session-specific fields
    Encryption,
    Compression,
    Scheme,
    EncryptionOptions,
    CompressionOptions,
    SchemeOptions,
    // Extra (sometimes unique) fields 
    Type,
    Uri,
    // Handle unknown fields
    Other(String)
}

impl Deserialize for FieldHelper {
    fn deserialize<D>(deserializer: &mut D) -> Result<FieldHelper, D::Error>
        where D: Deserializer,
    {
        struct FieldVisitor;

        impl Visitor for FieldVisitor {
            type Value = FieldHelper;


            fn visit_str<E>(&mut self, value: &str) -> Result<FieldHelper, E>
                where E: DeError,
            {
                use self::FieldHelper::*;
                Ok(match value {
                    "to" => To,
                    "from" => From,
                    "pp" => Pp,
                    "id" => Id,
                    "metadata" => Metadata,
                    "content" => Content,   // Message
                    "event" => Event,       // Notification
                    "method" => Method,     // Command
                    "state" => State,       // Session
                    "encryption" => Encryption,
                    "compression" => Compression,
                    "scheme" => Scheme,
                    "encryptionOptions" => EncryptionOptions,
                    "compressionOptions" => CompressionOptions,
                    "schemeOptions" => SchemeOptions,
                    "type" => Type,
                    "uri" => Uri,

                    _ => Other(value.to_owned()),
                })
            }
        }

        deserializer.deserialize_str(FieldVisitor)
    }
}

/// Deserialization implementation distinguishes the specific type of 'frame'
/// being received.
impl Deserialize for SealedEnvelope {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        struct EnvelopeVisitor;

        impl Visitor for EnvelopeVisitor {
            type Value = SealedEnvelope;

            fn visit_map<V>(&mut self, mut visitor: V)
                            -> Result<SealedEnvelope, V::Error>
                where V: MapVisitor,
            {
                let mut to          = None;
                let mut from        = None;
                let mut pp          = None;
                let mut id          = None;
                let mut metadata    = None;
                let mut content     = None;
                let mut event       = None;
                let mut method      = None;
                let mut state       = None;
                let mut encryption  = None;
                let mut compression = None;
                let mut scheme      = None;
                let mut e_options   = None;
                let mut c_options   = None;
                let mut s_options   = None;
                let mut mime_type   = None;
                let mut uri         = None;
                let mut other       = Map::new();

                use self::FieldHelper::*;
                while let Some(field) = visitor.visit_key()? {
                    match field {
                        To => to = Some(visitor.visit_value()?),
                        From => from = Some(visitor.visit_value()?),
                        Pp => pp = Some(visitor.visit_value()?),
                        Id => id = Some(visitor.visit_value()?),
                        Metadata => metadata = Some(visitor.visit_value()?),

                        Content => content = Some(visitor.visit_value()?),
                        Event => event = Some(visitor.visit_value()?),
                        Method => method = Some(visitor.visit_value()?),
                        State => state = Some(visitor.visit_value()?),

                        Encryption => encryption = Some(visitor.visit_value()?),
                        Compression => compression = Some(visitor.visit_value()?),
                        Scheme => scheme = Some(visitor.visit_value()?),
                        EncryptionOptions => e_options = Some(visitor.visit_value()?),
                        CompressionOptions => c_options = Some(visitor.visit_value()?),
                        SchemeOptions => s_options = Some(visitor.visit_value()?),

                        Type => mime_type = Some(visitor.visit_value()?),
                        Uri => uri = Some(visitor.visit_value()?),
                        Other(key) => {
                            other.insert(key, visitor.visit_value()?);
                        }
                    }
                }
                visitor.end()?;

                // TODO: Match all fields which are at some point required.
                Ok(match (content, event, method, state) {
                    (Some(content), None, None, None) => {
                        SealedEnvelope::Message(Message {
                            to: to,
                            from: from,
                            pp: pp,
                            id: id,
                            metadata: metadata,
                            mime_type: mime_type,
                            content: content,
                        })
                    }
                    (None, Some(event), None, None) => {
                        SealedEnvelope::Notification(Notification {
                            to: to,
                            from: from,
                            pp: pp,
                            id: id,
                            metadata: metadata,
                            event: event,
                        })
                    }
                    (None, None, Some(method), None) => {
                        SealedEnvelope::Command(Command {
                            to: to,
                            from: from,
                            pp: pp,
                            id: id,
                            metadata: metadata,
                            mime_type: mime_type,
                            method: method,
                            uri: uri,
                        })
                    }
                    (None, None, None, Some(state)) => {
                        //unimplemented!()
                        match (encryption, compression, scheme) {
                            (None, None, None) => {
                                SealedEnvelope::SessionReq(SessionRequest {
                                    to: to,
                                    from: from,
                                    pp: pp,
                                    id: id,
                                    metadata: metadata,
                                    state: state,
                                    encryption_options: e_options,
                                    compression_options: c_options,
                                    scheme_options: s_options,
                                })
                            }
                            _ => {
                                SealedEnvelope::SessionRes(SessionResponse {
                                    to: to,
                                    from: from,
                                    pp: pp,
                                    id: id,
                                    metadata: metadata,
                                    state: state,
                                    encryption: encryption,
                                    compression: compression,
                                    scheme: scheme,
                                })
                            }
                        }
                    }
                    (content, event, method, state) => {
                        unimplemented!()
                        //use serde_json::to_value;
                        //id.map(|id| other.insert("id".into(), to_value(id)));
                        //event.map(|event| other.insert("event".into(), to_value(event)));
                        //method.map(
                        //  |method| other.insert("method".into(), to_value(method)));
                        //Envelope::Other(other)
                    }
                })
            }
        }

        deserializer.deserialize_map(EnvelopeVisitor)
    }
}

