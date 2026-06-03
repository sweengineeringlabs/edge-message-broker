//! SAF layer — message broker public facade.

mod broker_svc;

#[cfg(feature = "nats")]
pub use crate::api::broker::nats::NatsMessageBroker;
#[cfg(feature = "tokio-rt")]
pub use crate::api::broker::r#in::InMemoryMessageBroker;
pub use crate::api::broker::Message;
pub use crate::api::broker::MessageBroker;
pub use crate::api::broker::MessageStream;
pub use crate::api::error::BrokerError;
pub use crate::api::types::ApplicationConfigBuilder;
pub use crate::api::types::BackendKind;
pub use crate::api::types::BrokerSvc;
pub use crate::api::types::MessageBrokerConfig;
pub use crate::api::types::Validator;
