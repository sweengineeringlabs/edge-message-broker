//! [`Message`] — the unit of exchange for [`crate::MessageBroker`].

use std::collections::HashMap;

/// A message payload with optional metadata headers.
///
/// `Message` is the currency passed between producers and consumers.  It
/// carries raw bytes and an optional key-value header map for routing hints,
/// content-type annotations, or correlation IDs.
///
/// # Examples
///
/// ```rust
/// use swe_edge_message_broker::Message;
///
/// // Simple payload — no headers.
/// let msg = Message::new(b"order.created:{\"id\":1}".as_ref());
/// assert_eq!(msg.payload.as_slice(), b"order.created:{\"id\":1}");
/// assert!(msg.headers.is_empty());
///
/// // With routing metadata.
/// let msg = Message::with_headers(
///     b"hello".as_ref(),
///     [("content-type".to_string(), "text/plain".to_string())].into(),
/// );
/// assert_eq!(msg.headers.get("content-type").map(String::as_str), Some("text/plain"));
/// ```
#[derive(Debug, Clone)]
pub struct Message {
    /// Payload bytes.
    pub payload: Vec<u8>,
    /// Optional key-value metadata headers.
    pub headers: HashMap<String, String>,
}
