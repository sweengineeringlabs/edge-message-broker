# Changelog

## 0.3.8 (2026-09-12)
- **Breaking:** migrated the `configbuilder` dependency from the git-sourced
  `swe-edge-configbuilder` to the officially published `configbuilder` crate
  (crates.io, owned by `sweengineeringlabs`). `OptionalSection`/`ConfigError`/
  `FeatureMetadata`/`FeatureStateOps`/`BuilderFinalizer` have identical shapes
  between the two crates, so this is a dependency-source swap only — no
  behavior change. Consumers must depend on `configbuilder` (not
  `swe-edge-configbuilder`) to call `MessageBrokerConfig`'s `OptionalSection`
  impl or `BrokerSvc::create_config_builder`'s returned builder, since the two
  crates' traits are nominally distinct types even though structurally
  identical. Matches the same migration applied to `edge-a2ac`, `edge-runtime`,
  and `edge-data`.

## 0.1.0 (2026-05-18)
- Initial release: `MessageBroker` trait, `Message`, `BrokerError`, `MessageStream`.
- `InMemoryMessageBroker` (tokio-rt): tokio broadcast-channel backend.
- `NatsMessageBroker` (nats): async-nats pure-Rust backend.
