/// Private helper that reflects the structure of the JSON.
/// Notification event
#[derive(Serialize, Deserialize)]
enum EventHelper {
    #[serde(rename="accepted")]     Accepted,
    #[serde(rename="validated")]    Validated,
    #[serde(rename="authorized")]   Authorized,
    #[serde(rename="dispatched")]   Dispatched,
    #[serde(rename="received")]     Received,
    #[serde(rename="consumed")]     Consumed,
    #[serde(rename="failed")]       Failed,
}

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
    Reason,
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
                    "reason" => Reason,

                    _ => Other(value.to_owned()),
                })
            }
        }

        deserializer.deserialize_str(FieldVisitor)
    }
}

