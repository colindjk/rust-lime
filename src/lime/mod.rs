
/// -- Global Constants --
static DELIMITER : u8 = b'\n' as u8;

pub use self::message::{
    Request,
    Response,
    Error
};

pub mod message;
pub mod envelope;
