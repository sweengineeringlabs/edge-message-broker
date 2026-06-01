//! Integration tests for the in-memory message broker API marker.
// @allow: no_mocks_in_integration — InMemoryMessageBroker is a production
// feature (tokio broadcast channel broker), not a mock. It is the primary
// non-NATS broker implementation shipped with the crate.

use swe_edge_message_broker::InMemoryMessageBroker;

/// @covers: InMemoryMessageBroker — marker type is constructible
#[test]
fn test_in_memory_message_broker_api_marker_is_constructible() {
    // @allow: no_mocks_in_integration
    let _ = InMemoryMessageBroker;
}
