extern crate protocol_manager;
extern crate serde_json;

use protocol_manager::lime::SealedEnvelope;
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
    use protocol_manager::lime::NotificationEvent::*;

    let notification_json = r#"{
            "id": "48600604-ce09-479c-b985-1195b196fe8e",
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
    assert_eq!(notification.id,
               "48600604-ce09-479c-b985-1195b196fe8e".to_string());
    assert_eq!(notification.to,
               Some("heisenberg@breakingbad.com/bedroom".to_string()));
    assert_eq!(notification.from,
               Some("skyler@breakingbad.com/bedroom".to_string()));
    assert_eq!(notification.event, Received);
}

#[test]
fn notification_failure() {
    use protocol_manager::lime::NotificationEvent::*;
    use protocol_manager::lime::ErrReason;

    let notification_json = r#"{
            "id": "9d0c4fea-75c7-432a-a164-c1a219bc17a8",
            "to": "skyler@breakingbad.com/bedroom",
            "event": "failed",
            "reason": {
                "code": 42,
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
    assert_eq!(notification.id,
               "9d0c4fea-75c7-432a-a164-c1a219bc17a8".to_string());
    assert_eq!(notification.to,
               Some("skyler@breakingbad.com/bedroom".to_string()));
    assert_eq!(notification.from, None);
    assert_eq!(notification.event,
            Failed(ErrReason { code: 42, description:
                Some("The message destination was not found"
                     .to_string()) }));
}
