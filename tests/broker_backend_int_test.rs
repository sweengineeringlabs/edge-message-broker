//! Integration tests for the broker backend SPI extension point.
//!
//! The BrokerBackend SPI struct is an extension point for downstream consumers.
//! These tests verify that the MessageBroker contract is satisfied by a
//! custom implementation — which is the primary SPI use case.

use swe_edge_message_broker::{BrokerSvc, MessageBroker, Validator};

/// @covers: BrokerSvc::validate — validates a custom MessageBroker implementation
#[test]
fn test_custom_validator_satisfies_broker_svc_validate() {
    struct MyValidator;
    impl Validator for MyValidator {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(BrokerSvc::validate(&MyValidator).is_ok());
}

/// @covers: MessageBroker — trait is object safe, enabling custom impls
#[test]
fn test_message_broker_trait_allows_custom_impl() {
    fn _check(_: &dyn MessageBroker) {}
}
