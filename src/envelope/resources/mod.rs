
pub mod uri;

pub enum Resource {
    Account(Account),
    Capability(Capability),
}

/// Represents the user information as a series of options.
#[derive(Debug)]
pub struct Account {
    full_name: Option<String>,
    address: Option<String>,
    city: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    photo_uri: Option<String>,
    cell_phone_number: Option<String>,
    is_temporary: Option<bool>,
    password: Option<String>,
    old_password: Option<String>,
    inbox_size: Option<String>,
    allow_guest_sender: Option<bool>,
    allow_unknown_sender: Option<bool>,
    store_message_content: Option<bool>,
}

pub struct Capability {

}

