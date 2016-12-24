/// This section includes all info pertaining to parsing the LIME protocol
/// into messages.

/// This will be split into many files when I get the understanding needed.

// TODO: Create a client connection struct which will handle default values for
// protocol fields.

//use tokio::io::{
    //Codec, EasyBuf
//};

//use json::ser::{to_vec};
//use json::de::{from_slice};

use serde_json::Map;

pub type Node = String;
pub type MsgID = String;
pub type TimeStamp = u64;

/// Trait for all envelope related types.
/// TODO: Include 'pp' and 'metadata'
pub trait Envelope {
    fn id(&self) -> Option<MsgID>;
    fn to(&self) -> Option<Node>;
    fn from(&self) -> Option<Node>;
    //fn pp(&self) -> Option<Node>;
    //fn metadata(&self) -> Option<Node>;

    fn set_id(&self) -> Option<MsgID>;
    fn set_to(&self) -> Option<Node>;
    fn set_from(&self) -> Option<Node>;
    //fn set_pp(&self) -> Option<Node>;
    //fn set_metadata(&self) -> Option<Node>;
}

macro_rules! impl_Envelope(
    ($kind: ident, $ty: ty, $ty_some: expr, $ty_none: expr) => (
        impl Envelope for $kind {
            type Ty = $ty;

            fn into_inner(self) -> Map {
                self.map
            }

            fn id(&self) -> Option<&str> {
                self.map.get("id").as_str()
            }

            fn to(&self) -> Option<&str> {
                self.map.get("to").as_str()
            }

            fn from(&self) -> Option<&str> {
                self.map.get("from").as_str()
            }

            fn envelope_type(&self) -> Option<$ty> {
                match self.map.get("type") {
                    Some(ty) => ($ty_some)(ty),
                    None => $ty_none
                }
            }

            fn set_id(&mut self, id: Option<String>) {
                if let Some(id) = id {
                    self.set("id".into(), None, id);
                } else {
                    self.remove("id");
                }
            }

            fn set_to(&mut self, to: Option<String>) {
                if let Some(to) = to {
                    self.set("to".into(), None, to);
                } else {
                    self.remove("to", None);
                }
            }

            fn set_from(&mut self, from: Option<String>) {
                if let Some(from) = from {
                    self.set("from".into(), None, from);
                } else {
                    self.remove("from");
                }
            }

        }

    );
);

/// Outlines the kinds of envelopes one can receive.
/// TODO: Figure out if values as '&str' efficient / possible / worth.
#[derive(Debug, Serialize, Deserialize)]
pub enum EnvelopeKind {
    Message {
        mime_type: String,
        content: String,
    },
    Notification {
        event: NotificationEvent,
    },
    Command {
        mime_type: String,
        method: CommandMethod,
        uri: String,

    },
    Session {

    }
}

//fn error_reply(&self,
//               ty: ::stanzas::ErrorType,
//               cond: ::stanzas::DefinedCondition,
//               text: Option<String>) -> $kind
//{
//    let to = self.from().map(|x| x.into());
//    let id = self.id().unwrap_or("").into();
//    let ty = ty.attr_string().into();
//
//    let mut reply = $kind {
//        elem: Value::new($name.into(), Some(ns::JABBER_CLIENT.into()),
//                                vec![("type".into(), None, "error".into()),
//                                     ("id".into(), None, id)])
//    };
//    {
//        let error = reply.tag(xml::Element::new("error".into(),
//                                                Some(ns::JABBER_CLIENT.into()),
//                                                vec![("type".into(), None, ty)]))
//                         .tag_stay(cond.element());
//        if let Some(text) = text {
//            error.tag(xml::Element::new("text".into(),
//                                        Some(ns::STANZA_ERRORS.into()), vec![]))
//                 .text(text);
//        }
//    }
//    reply.set_to(to);
//    reply
//}
