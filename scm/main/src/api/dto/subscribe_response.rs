//! Response for [`crate::MessageBroker::subscribe`].

use crate::api::MessageStream;

/// Response for [`crate::MessageBroker::subscribe`].
pub struct SubscribeResponse {
    /// The stream of incoming messages.
    pub stream: MessageStream,
}
