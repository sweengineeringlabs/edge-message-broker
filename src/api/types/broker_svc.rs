//! `BrokerSvc` — factory type declaration.

/// Message broker factory and configuration entrypoint.
///
/// Methods on this type are the sole public surface for constructing broker
/// instances and config builders.  Consumers call `BrokerSvc::in_memory_broker()`
/// rather than naming concrete implementation types.
pub struct BrokerSvc;
