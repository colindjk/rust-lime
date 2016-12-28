use serde::{Deserialize, Deserializer};
use serde::de::{Visitor, Error as DeError};

use lime::{ErrReason};

/// Private helper that reflects the structure of the JSON.
/// Notification event
#[derive(Serialize, Deserialize)]
pub enum NotificationEventHelper {
    #[serde(rename="accepted")]     Accepted,
    #[serde(rename="validated")]    Validated,
    #[serde(rename="authorized")]   Authorized,
    #[serde(rename="dispatched")]   Dispatched,
    #[serde(rename="received")]     Received,
    #[serde(rename="consumed")]     Consumed,
    #[serde(rename="failed")]       Failed,
}

use lime::notification::NotificationEvent;

pub fn into_event(helper: NotificationEventHelper,
                  reason: Option<ErrReason>) -> NotificationEvent {
    use lime::notification::NotificationEvent::*;
    match (helper, reason) {
        (NotificationEventHelper::Accepted, None) => Accepted,
        (NotificationEventHelper::Validated, None) => Validated,
        (NotificationEventHelper::Authorized, None) => Authorized,
        (NotificationEventHelper::Dispatched, None) => Dispatched,
        (NotificationEventHelper::Received, None) => Received,
        (NotificationEventHelper::Consumed, None) => Consumed,
        (NotificationEventHelper::Failed, Some(reason)) => Failed(reason),
        (NotificationEventHelper::Failed, None) => {
            unimplemented!()
        },
        (_, Some(_)) => {
            unimplemented!()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum CommandStatusHelper {
    #[serde(rename="success")] Success,
    #[serde(rename="failure")] Failure,
}

use lime::command::CommandStatus;

pub fn into_status(helper: Option<CommandStatusHelper>,
                   reason: Option<ErrReason>) -> Option<CommandStatus> {
    use lime::command::CommandStatus::*;
    match (helper, reason) {
        (Some(CommandStatusHelper::Success), None) => Some(Success),
        (Some(CommandStatusHelper::Failure), Some(rsn)) => Some(Failure(rsn)),
        (_, Some(_)) => {
            panic!("Invalid Envelope Format: Non-Failure cmd \
                    response with ErrReason.")
        },
        (Some(CommandStatusHelper::Failure), None) => {
            panic!("Invalid Envelope Format: Failed cmd \
                    response with no ErrReason given.")
        },
        (None, _) => None, // TODO: Some(reason)?
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionStateHelper {
    #[serde(rename="new")]              New,
    #[serde(rename="negotiating")]      Negotiating,
    #[serde(rename="authenticating")]   Authenticating,
    #[serde(rename="established")]      Established,
    #[serde(rename="finishing")]        Finishing,
    #[serde(rename="finished")]         Finished,
    #[serde(rename="failed")]           Failed,
}

use lime::session::SessionState;

pub fn into_state(helper: SessionStateHelper,
                  reason: Option<ErrReason>) -> SessionState {
    use lime::session::SessionState::*;
    match (helper, reason) {
        (SessionStateHelper::New, None) => New,
        (SessionStateHelper::Negotiating, None) => Negotiating,
        (SessionStateHelper::Authenticating, None) => Authenticating,
        (SessionStateHelper::Established, None) => Established,
        (SessionStateHelper::Finishing, None) => Finishing,
        (SessionStateHelper::Finished, None) => Finished,
        (SessionStateHelper::Failed, Some(rsn)) => Failed(rsn),
        (_, _) => panic!("No reason given for failure or vise-versa"),
    }
}

/// Contains all known fields an Envelope can contain.
pub enum FieldHelper {
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
    Status,
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
                    "status" => Status,

                    _ => Other(value.to_owned()),
                })
            }
        }

        deserializer.deserialize_str(FieldVisitor)
    }
}

