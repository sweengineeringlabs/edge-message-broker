//! Integration tests for [`MessageBroker`] trait object safety.

use swe_edge_message_broker::MessageBroker;

/// @covers: MessageBroker — trait is object safe
#[test]
fn test_message_broker_is_object_safe() {
    fn _check(_: &dyn MessageBroker) {}
}
