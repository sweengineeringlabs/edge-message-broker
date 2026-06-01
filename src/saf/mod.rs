//! SAF layer — message broker public facade.

mod broker_svc;

pub use crate::api::types::ApplicationConfigBuilder;
pub use crate::api::types::BrokerSvc;
pub use crate::api::error::BrokerError;
pub use crate::api::broker::Message;
pub use crate::api::broker::MessageBroker;
pub use crate::api::broker::MessageStream;
pub use crate::api::types::Validator;
#[cfg(feature = "tokio-rt")]
pub use crate::api::broker::r#in::InMemoryMessageBroker;
#[cfg(feature = "nats")]
pub use crate::api::broker::nats::NatsMessageBroker;
