//! [`Message`] construction. Kept in `core/`, not `api/`: `Message` itself is
//! a value type declared in `api/vo/`, but its constructors are
//! implementation, and `api/` is a pure declaration layer (`api_no_inherent_impl`).

use std::collections::HashMap;

use bytes::Bytes;

use crate::api::Message;

impl Message {
    /// Construct a message from raw bytes with no headers.
    pub fn new(payload: impl Into<Bytes>) -> Self {
        Self {
            payload: payload.into().to_vec(),
            headers: HashMap::new(),
        }
    }

    /// Construct a message with headers.
    pub fn with_headers(payload: impl Into<Bytes>, headers: HashMap<String, String>) -> Self {
        Self {
            payload: payload.into().to_vec(),
            headers,
        }
    }
}
