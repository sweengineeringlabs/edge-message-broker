//! SAF layer — message broker public facade.

mod broker_svc;

pub use crate::api::error::BrokerError;
pub use crate::api::traits::MessageBroker;
pub use crate::api::traits::Validator;
pub use crate::api::types::ApplicationConfigBuilder;
pub use crate::api::types::BackendKind;
pub use crate::api::types::BrokerSvc;
pub use crate::api::types::Message;
pub use crate::api::types::MessageBrokerConfig;
pub use crate::api::types::MessageStream;
