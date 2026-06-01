//! Integration tests for [`Message`].

use swe_edge_message_broker::Message;

/// @covers: Message::new
#[test]
fn test_new_creates_message_with_empty_headers() {
    let m = Message::new(b"hello".as_ref());
    assert_eq!(m.payload.as_ref(), b"hello");
    assert!(m.headers.is_empty());
}

/// @covers: Message::with_headers
#[test]
fn test_with_headers_stores_provided_headers() {
    use std::collections::HashMap;
    let mut h = HashMap::new();
    h.insert("content-type".into(), "application/json".into());
    let m = Message::with_headers(b"{}".as_ref(), h);
    assert_eq!(
        m.headers.get("content-type").map(String::as_str),
        Some("application/json")
    );
}

/// @covers: Message::new — clone produces independent copy
#[test]
fn test_message_clone_produces_independent_copy() {
    use bytes::Bytes;
    let m = Message::new(b"data".as_ref());
    let m2 = m.clone();
    assert_eq!(m.payload, m2.payload);
    assert_eq!(m2.payload, Bytes::from_static(b"data"));
}
