// SerDe section

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, MapVisitor};
use serde_json::Map;

use envelope::{
    Envelope,
    Message,
    Notification,
    Command,
    Session, 
};

use envelope::helper::*;

/// Deserialization implementation distinguishes the specific type of 'frame'
/// being received.
impl Deserialize for Envelope {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        struct EnvelopeVisitor;

        impl Visitor for EnvelopeVisitor {
            type Value = Envelope;

            fn visit_map<V>(&mut self, mut vis: V)
                            -> Result<Envelope, V::Error>
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

                let mut status      = None;
                let mut encryption  = None;
                let mut compression = None;
                let mut scheme      = None;
                let mut e_options   = None;
                let mut c_options   = None;
                let mut s_options   = None;

                let mut mime_type   = None;
                let mut uri         = None;
                let mut reason      = None;
                let mut other       = Map::new();

                use envelope::helper::FieldHelper::*;
                while let Some(field) = vis.visit_key()? {
                    match field {
                        To => to = Some(vis.visit_value()?),
                        From => from = Some(vis.visit_value()?),
                        Pp => pp = Some(vis.visit_value()?),
                        Id => id = Some(vis.visit_value()?),
                        Metadata => metadata = Some(vis.visit_value()?),

                        Content => content = Some(vis.visit_value()?),
                        Event => event = Some(vis.visit_value()?),
                        Method => method = Some(vis.visit_value()?),
                        State => state = Some(vis.visit_value()?),

                        Encryption => encryption = Some(vis.visit_value()?),
                        Compression => compression = Some(vis.visit_value()?),
                        Scheme => scheme = Some(vis.visit_value()?),
                        EncryptionOptions => e_options = Some(vis.visit_value()?),
                        CompressionOptions => c_options = Some(vis.visit_value()?),
                        SchemeOptions => s_options = Some(vis.visit_value()?),

                        Type => mime_type = Some(vis.visit_value()?),
                        Uri => uri = Some(vis.visit_value()?),
                        Reason => reason = Some(vis.visit_value()?),
                        Status => status = Some(vis.visit_value()?),
                        Other(key) => {
                            other.insert(key, vis.visit_value()?);
                        }
                    }
                }
                vis.end()?;

                // TODO: Match all fields which are at some point required.
                Ok(match (content, event, method, state, id, mime_type) {
                    (Some(content), None, None, None, id, Some(mime_type)) => {
                        Envelope::Message(Message {
                            to: to,
                            from: from,
                            pp: pp,
                            id: id,
                            metadata: metadata,
                            mime_type: mime_type,
                            content: content,
                        })
                    }
                    (None, Some(event), None, None, Some(id), None) => {
                        let event = into_event(event, reason);
                        Envelope::Notification(Notification {
                            to: to,
                            from: from,
                            pp: pp,
                            id: id,
                            metadata: metadata,
                            event: event,
                        })
                    }
                    (None, None, Some(method), None, id, mime_type) => {
                        let status = into_status(status, reason);
                        Envelope::Command(Command {
                            to: to,
                            from: from,
                            pp: pp,
                            id: id,
                            metadata: metadata,
                            mime_type: mime_type,
                            method: method,
                            status: status,
                            uri: uri,
                        })
                    }
                    (None, None, None, Some(state), Some(id), None) => {
                        let state = into_state(state, reason);
                        Envelope::Session(Session {
                            to: to,
                            from: from,
                            pp: pp,
                            id: id,
                            metadata: metadata,
                            state: state,
                            encryption_options: e_options,
                            compression_options: c_options,
                            scheme_options: s_options,
                            encryption: encryption,
                            compression: compression,
                            scheme: scheme,
                        })
                    }
                    _ => unimplemented!(), // take care of this at some point
                })
            }
        }

        deserializer.deserialize_map(EnvelopeVisitor)
    }
}

impl Serialize for Envelope {
    fn serialize<S>(&self, serializer: &mut S)
            -> Result<(), S::Error> where S: Serializer
    {
        use envelope::Envelope::*;
        match *self {
            Message(ref val)      => val.serialize(serializer),
            Notification(ref val) => val.serialize(serializer),
            //Command(ref val)        => val.serialize(serializer),
            //SessionReq(ref val)     => val.serialize(serializer),
            //SessionRes(ref val)     => val.serialize(serializer),
            //Unknown(ref val)        => val.serialize(serializer),
            _ => unimplemented!()
        }
    }
}

