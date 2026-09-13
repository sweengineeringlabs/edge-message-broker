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
- **Breaking:** `MessageBroker::publish`/`subscribe`/`health_check` now take a
  single `*Request` struct (`PublishRequest`/`SubscribeRequest`/
  `HealthCheckRequest`) instead of positional arguments, and `subscribe`
  returns `SubscribeResponse { stream }` instead of a bare `MessageStream`.
  `PublishRequest.message` is `Arc<Message>` (shared ownership), not `Message`
  by value.
- **Breaking:** added `MessageBroker::validator(&self, ValidatorRequest) ->
  Result<ValidatorResponse, BrokerError>` — every broker backend now exposes a
  real handle to its own config `Validator`, letting a caller revalidate a
  live broker's configuration (e.g. for health dashboards or hot-reload
  checks) without naming the concrete config type.
- **Breaking:** `Validator::validate` now takes a `ValidationRequest` marker
  and returns `Result<(), ValidationError>`; `ValidationError` carries
  `Vec<String>` violations (itemized diagnostics) instead of a bare `String`.
- **Breaking:** `Message.payload` is `Vec<u8>`, not `bytes::Bytes`.
  `AsRef<[u8]>`/indexing/equality against byte slices continue to work
  unchanged; direct `Bytes` comparisons need `.to_vec()` or `.as_slice()`
  adjustments.
- **Breaking:** removed the dead `ApplicationConfigBuilder` type alias — zero
  real callers anywhere in this crate or `edge-runtime` (confirmed via
  workspace-wide search), the same class of cleanup `edge-data`'s own
  CHANGELOG `[0.3.0]` recorded for the identical pattern.
- Internal: `api/` is now a pure declaration layer (all `impl` blocks moved to
  `core/`), split `api/types/` into `api/vo/` (plain value objects) and
  `api/dto/` (request/response types), added `saf/message_broker_svc_factory.rs`
  and `saf/validator_svc_factory.rs` (one composition site per `api/` trait),
  and removed the empty `main/src/gateway/` scaffold (two stub submodules
  re-exporting `saf::*`, no gateway logic — disallowed in a lib crate).

## 0.1.0 (2026-05-18)
- Initial release: `MessageBroker` trait, `Message`, `BrokerError`, `MessageStream`.
- `InMemoryMessageBroker` (tokio-rt): tokio broadcast-channel backend.
- `NatsMessageBroker` (nats): async-nats pure-Rust backend.
