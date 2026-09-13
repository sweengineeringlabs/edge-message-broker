//! Composition site for [`MessageBroker`]. The constructor
//! ([`BrokerSvc::create_message_broker`](crate::BrokerSvc::create_message_broker))
//! is a method on [`BrokerSvc`](crate::BrokerSvc) in `broker_svc.rs`, not a
//! free function here (`layers_no_free_standing_fn`).

pub use crate::api::MessageBroker;
