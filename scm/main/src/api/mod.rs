//! API layer — public trait contracts and value types.

mod dto;
mod error;
mod traits;
mod types;
mod vo;

pub use dto::{
    HealthCheckRequest, PublishRequest, SubscribeRequest, SubscribeResponse, ValidationRequest,
    ValidatorRequest, ValidatorResponse,
};
pub use error::{BrokerError, ValidationError};
pub use traits::{MessageBroker, Validator};
pub use types::{BrokerFuture, BrokerSvc, MessageBrokerConfig, MessageStream};
pub use vo::{BackendKind, Message};
