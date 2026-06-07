//! [`MessageStream`] — ordered stream of messages from a broker subscription.

use std::pin::Pin;

use futures::Stream;

use crate::api::error::broker_error::BrokerError;
use crate::api::types::message::Message;

/// An ordered stream of messages received from a [`crate::MessageBroker`] subscription.
pub type MessageStream = Pin<Box<dyn Stream<Item = Result<Message, BrokerError>> + Send>>;
