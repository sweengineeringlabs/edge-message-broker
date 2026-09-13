//! Composition site for [`Validator`]. The constructor
//! ([`BrokerSvc::create_validator`](crate::BrokerSvc::create_validator)) is a
//! method on [`BrokerSvc`](crate::BrokerSvc) in `broker_svc.rs`, not a free
//! function here (`layers_no_free_standing_fn`).

pub use crate::api::Validator;
