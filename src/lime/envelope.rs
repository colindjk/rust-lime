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

pub type Node = String;
pub type MsgID = String;
pub type TimeStamp = u64;

/// Trait for all envelope related types.
/// TODO: Include 'pp' and 'metadata'
/// TODO: Convert to MIME
pub trait Envelope {
    type Ty;

    fn unique_field<'a>() -> &'a str;

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
    ($kind: ident, $ty: ty, $ty_some: expr, $ty_none: expr, $unique_field: expr) => (
        impl Envelope for $kind {
            type Ty = $ty;

            fn unique_field<'a>() -> &'a str { $unique_field }

            fn id(&self) -> Option<&str> { self.id.as_ref().map(|s| &**s) }
            fn to(&self) -> Option<&str> { self.to.as_ref().map(|s| &**s) }
            fn from(&self) -> Option<&str> { self.from.as_ref().map(|s| &**s) }
            fn set_id(&mut self, id: Option<String>) { self.id = id; }
            fn set_to(&mut self, to: Option<String>) { self.to = to; }
            fn set_from(&mut self, from: Option<String>) { self.from = from; }

        }

    );
);

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
