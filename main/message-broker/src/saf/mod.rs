//! SAF layer — message broker public facade.

mod broker_svc;

pub use crate::api::broker::Message;
pub use crate::api::broker::MessageBroker;
pub use crate::api::broker::MessageStream;
pub use crate::api::error::BrokerError;
pub use crate::api::types::ApplicationConfigBuilder;
pub use crate::api::types::BackendKind;
pub use crate::api::types::BrokerSvc;
pub use crate::api::types::MessageBrokerConfig;
pub use crate::api::types::Validator;
