//! `ApplicationConfigBuilder` — maps to `config/application.toml`.

/// Builder type for message broker application configuration.
///
/// Alias for [`swe_edge_configbuilder::ConfigBuilderImpl`] pre-seeded with
/// this crate's name and version via [`crate::BrokerSvc::create_config_builder`].
pub type ApplicationConfigBuilder = swe_edge_configbuilder::ConfigBuilderImpl;
