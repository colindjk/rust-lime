extern crate protocol_manager as rust_lime;
extern crate serde_json;

use rust_lime::envelope::SealedEnvelope;
use serde_json::from_str;
use serde_json::Value::*;

#[test]
fn message_basic() {
    let message_json = r#"{
            "from": "skyler@breakingbad.com/bedroom",
            "to": "ww@breakingbad.com",
            "type": "text/plain",
            "content": "Walter, are you in danger?"    
        }"#;
    let message : SealedEnvelope = from_str(message_json).unwrap();
    let message = if let SealedEnvelope::Message(msg) = message {
        msg
    } else {
        panic!("Non-message envelope parsed from json with content")
    };
    assert_eq!(message.to,
               Some("ww@breakingbad.com".to_string()));
    assert_eq!(message.from,
               Some("skyler@breakingbad.com/bedroom".to_string()));
    assert_eq!(message.content,
               String("Walter, are you in danger?".to_string()));
}

#[test]
fn notification_basic() {
    use rust_lime::envelope::NotificationEvent::*;

    let notification_json = r#"{
            "id": "54321",
            "from": "skyler@breakingbad.com/bedroom",
            "to": "heisenberg@breakingbad.com/bedroom",
            "event": "received"
        }"#;
    let notification : SealedEnvelope = from_str(notification_json).unwrap();
    let notification =
            if let SealedEnvelope::Notification(notify) = notification {
        notify
    } else {
        panic!("Non-notification envelope parsed from json with event")
    };
    assert_eq!(notification.id, 54321);
    assert_eq!(notification.to,
               Some("heisenberg@breakingbad.com/bedroom".to_string()));
    assert_eq!(notification.from,
               Some("skyler@breakingbad.com/bedroom".to_string()));
    assert_eq!(notification.event, Received);
}

#[test]
fn notification_failure() {
    use rust_lime::envelope::NotificationEvent::*;
    use rust_lime::envelope::ErrReason;

    let notification_json = r#"{
            "id": "12345",
            "to": "skyler@breakingbad.com/bedroom",
            "event": "failed",
            "reason": {
                "code": 12,
                "description": "The message destination was not found"
            }
        }"#;
    let notification : SealedEnvelope = from_str(notification_json).unwrap();
    let notification =
            if let SealedEnvelope::Notification(notify) = notification {
        notify
    } else {
        panic!("Non-notification envelope parsed from json with event")
    };
    assert_eq!(notification.id, 12345);
    assert_eq!(notification.to,
               Some("skyler@breakingbad.com/bedroom".to_string()));
    assert_eq!(notification.from, None);

    use rust_lime::envelope::reason::ReasonCode::*;
    assert_eq!(notification.event, Failed(ErrReason {
        code: SessionRegistrationError,
        description: Some("The message destination was not found"
            .to_string()) }));
}
