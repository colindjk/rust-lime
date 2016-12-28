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
    //type Ty;

    //fn unique<'a>() -> &'a str;

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
            //type Ty = $ty;

            //fn unique<'a>() -> &'a str { $unique_field }

            fn id(&self) -> Option<&str> { self.id.as_ref().map(|s| &**s) }
            fn to(&self) -> Option<&str> { self.to.as_ref().map(|s| &**s) }
            fn from(&self) -> Option<&str> { self.from.as_ref().map(|s| &**s) }
            fn set_id(&mut self, id: Option<String>) { self.id = id; }
            fn set_to(&mut self, to: Option<String>) { self.to = to; }
            fn set_from(&mut self, from: Option<String>) { self.from = from; }

        }

    );
);

