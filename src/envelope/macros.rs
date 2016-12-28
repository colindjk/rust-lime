/// Macro for setting up an enum which will have number values representing
/// variants.
macro_rules! enum_number {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant = $value,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: ::serde::Serializer,
            {
                // Serialize the enum as a u64.
                serializer.serialize_u64(*self as u64)
            }
        }

        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer,
            {
                struct Visitor;

                impl ::serde::de::Visitor for Visitor {
                    type Value = $name;

                    fn visit_u64<E>(&mut self, value: u64) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        // Rust does not come with a simple way of converting a
                        // number to an enum, so use a big `match`.
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(
                                &format!("unknown {} value: {}",
                                stringify!($name), value))),
                        }
                    }
                }

                // Deserialize the enum from a u64.
                deserializer.deserialize_u64(Visitor)
            }
        }
    }
}

/// Quick and easy implementation for envelope types.
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

