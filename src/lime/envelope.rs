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

use serde_json::{ Map, Value };

//pub type Node = &'static str;
//pub type MsgID = &'static str;
pub type TimeStamp = u64;

/// Trait for all envelope related types.
/// TODO: Include 'pp' and 'metadata'
pub trait Envelope {
    type Ty;

    fn into_inner(self) -> Map<String, Value>;

    fn id(&self) -> Option<&str>;
    fn to(&self) -> Option<&str>;
    fn from(&self) -> Option<&str>;
    //fn pp(&self) -> Option<Node>;
    //fn metadata(&self) -> Option<Node>;

    fn set_id(&mut self, Option<String>);
    fn set_to(&mut self, Option<String>);
    fn set_from(&mut self, Option<String>);
    //fn set_pp(&self) -> Option<Node>;
    //fn set_metadata(&self) -> Option<Node>;
}

macro_rules! impl_Envelope(
    ($kind: ident, $ty: ty, $ty_some: expr, $ty_none: expr) => (
        impl Envelope for $kind {
            type Ty = $ty;

            fn into_inner(self) -> Map<String, Value> {
                self.map
            }

            fn id(&self) -> Option<&str> {
                if let Some(id) = self.map.get("id") {
                    id.as_str()
                } else {
                    None
                }
            }

            fn to(&self) -> Option<&str> {
                if let Some(to) = self.map.get("to") {
                    to.as_str()
                } else {
                    None
                }
            }

            fn from(&self) -> Option<&str> {
                if let Some(from) = self.map.get("from") {
                    from.as_str()
                } else {
                    None
                }
            }

            //fn envelope_type(&self) -> Option<$ty> {
                //match self.map.get("type") {
                    //Some(ty) => ($ty_some)(ty),
                    //None => $ty_none
                //}
            //}

            fn set_id(&mut self, id: Option<String>) {
                if let Some(id) = id {
                    self.map.insert("id".into(), Value::String(id));
                } else {
                    self.map.remove("id");
                }
            }

            fn set_to(&mut self, to: Option<String>) {
                if let Some(to) = to {
                    self.map.insert("to".into(), Value::String(to));
                } else {
                    self.map.remove("to");
                }
            }

            fn set_from(&mut self, from: Option<String>) {
                if let Some(from) = from {
                    self.map.insert("from".into(), Value::String(from));
                } else {
                    self.map.remove("from");
                }
            }

        }

    );
);

/// Outlines the kinds of envelopes one can receive.
/// TODO: Figure out if values as '&str' efficient / possible / worth.
//#[derive(Debug, Serialize, Deserialize)]
pub enum EnvelopeKind {
    Message {
        mime_type: String,
        content: String,
    },
    Notification {
        //event: NotificationEvent,
    },
    Command {
        mime_type: String,
        //method: CommandMethod,
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
