//! API value objects — message and configuration value types.

pub(crate) mod backend_kind;
#[allow(clippy::module_inception)]
pub(crate) mod message;
pub(crate) mod message_broker_config;

pub use backend_kind::BackendKind;
pub use message::Message;
pub use message_broker_config::MessageBrokerConfig;
