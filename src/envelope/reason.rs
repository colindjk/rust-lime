
/// When an Error occurs, this will exist.
/// TODO: Use this for other structs aside from just Notification.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Reason {
    pub code: ReasonCode,
    pub description: Option<String>
}

// TODO : Complete this
enum_number!(
ReasonCode {
    GeneralError = 0,
    SessionRegistrationError = 12,
    SessionAuthenticationFailed = 13,
});

