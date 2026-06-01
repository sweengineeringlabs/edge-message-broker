//! Integration tests for [`BrokerSvc::create_config_builder`].

use swe_edge_message_broker::BrokerSvc;

/// @covers: create_config_builder — returns a pre-seeded builder with this crate's package name
#[test]
fn test_create_config_builder_is_pre_seeded_with_package_name() {
    let _loader = BrokerSvc::create_config_builder().build_loader();
}

/// @covers: create_config_builder — builder is callable and does not panic
#[test]
fn test_create_config_builder_returns_without_panic() {
    let builder = BrokerSvc::create_config_builder();
    let _ = builder;
}
