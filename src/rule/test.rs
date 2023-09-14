use super::Message;

#[test]
fn test_message_serialize() {
    let msg = Message::new(10, "hello world".into());
    let json = serde_json::to_string(&msg).unwrap();
    assert_eq!(json, r#"{"code":10,"content":"hello world"}"#);

    let msg = Message::from_code(10);
    let json = serde_json::to_string(&msg).unwrap();
    assert_eq!(json, "10");

    let msg = Message::from_content("hello".into());
    let json = serde_json::to_string(&msg).unwrap();
    assert_eq!(json, r#""hello""#);
}
