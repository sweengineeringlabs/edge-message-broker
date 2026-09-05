# RFC-001 — Postgres/pgmq Message Broker Backend

**Status:** Proposed
**Date:** 2026-09-05
**Scope:** `swe-edge-message-broker` (this crate) — new `BackendKind::Postgres` variant; downstream implementation in `swe-edge-runtime-message-broker` (`runtime/message-broker`, edge-runtime repo)
**GitHub Issue:** sweengineeringlabs/edge-message-broker#5

---

## Motivation

This crate currently defines three backends via `BackendKind`:

```rust
// api/types/backend_kind.rs
pub enum BackendKind {
    InMemory,
    Nats,
    Kafka,
}
```

Some deployments already run Postgres and don't want to stand up NATS or Kafka purely for task/event queuing. `pgmq` (Tembo, open source) is a Postgres extension that implements SQS-style queue semantics (`send`, `read` with visibility timeout, `delete`, `archive`) directly on Postgres tables — no separate broker process.

---

## Proposed change

Add a `Postgres` variant to `BackendKind`, and a `PostgresMessageBroker` implementation behind a `postgres` feature flag in the downstream `swe-edge-runtime-message-broker` crate — following the same pattern as the existing `nats`/`kafka` backends there.

### Contract crate (this repo)

```rust
// api/types/backend_kind.rs
pub enum BackendKind {
    InMemory,
    Nats,
    Kafka,
    Postgres,   // new — requires `url` (Postgres DSN) and `queue_name`
}
```

### Runtime crate (`swe-edge-runtime-message-broker`)

```toml
[features]
postgres = ["dep:sqlx"]

[dependencies]
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio"], optional = true }
```

```
spi/broker/
  nats/          ← existing
  kafka/         ← existing
  postgres/      ← new
    mod.rs
    pgmq_message_broker.rs
```

Factory method, matching the existing pattern:

```rust
MessageBrokerFactory::nats("nats://localhost:4222").await?;
MessageBrokerFactory::kafka("localhost:9092").await?;
MessageBrokerFactory::postgres("postgres://localhost/db", "my_queue").await?;  // proposed
```

### Config

```toml
[broker]
backend    = "postgres"
url        = "postgres://user:pass@localhost/app"
queue_name = "edge_events"
```

### Requires

- `CREATE EXTENSION pgmq;` on the target database (out of band — neither crate manages extensions)
- Postgres user with privileges to call `pgmq.*` functions on the configured queue

---

## Comparison Matrix

| Aspect | InMemory | NATS | Kafka | Postgres (pgmq) |
|---|---|---|---|---|
| **External process** | None | NATS server | Kafka cluster | Postgres (likely already running) |
| **Delivery semantics** | Broadcast (all subscribers) | Pub/sub | Pub/sub + consumer groups | Queue (one consumer per message) |
| **Persistence** | None | Optional (JetStream) | Yes | Yes (transactional with the rest of your data) |
| **Throughput ceiling** | High (in-proc) | High | Very high | Moderate — bounded by Postgres write throughput |
| **Ops overhead** | None | Dedicated service | Dedicated cluster | None if Postgres already deployed |
| **Best for** | Tests, single-process | Multi-host pub/sub | High-throughput streaming | Task queues in Postgres-only deployments |

---

## Alternatives Considered

| Option | Verdict |
|---|---|
| `LISTEN`/`NOTIFY` (Postgres core, no extension) | Rejected as the sole mechanism — no durability, no ack/retry, 8000-byte payload cap. Fire-and-forget only. |
| Hand-rolled queue table (no `pgmq`) | Rejected — `pgmq` already solves visibility timeout, archive, and batch ops; reimplementing is wasted effort and more surface for bugs. |
| Treat as a database concern instead of message-broker | Rejected — the API surface (`send`/`read`/`ack`, visibility timeout) is queue semantics, matching this crate's existing `MessageBroker` trait contract, not a data-access pattern. |

---

## Consequences

**Positive**
- No new infrastructure for deployments that already run Postgres
- Transactional enqueue possible (write a row + enqueue a message in the same DB transaction)
- Durable by default (unlike `InMemory`, no extra config needed unlike NATS JetStream)
- Additive: existing `InMemory`/`Nats`/`Kafka` backends and the public `MessageBroker` trait are unchanged

**Negative**
- Queue semantics only — no broadcast/pub-sub fan-out to multiple independent consumer groups (unlike NATS/Kafka); each message goes to exactly one consumer
- Throughput bounded by Postgres — not a fit for high-frequency streaming workloads
- Requires the `pgmq` extension to be installed and enabled by the operator; neither crate can do that itself
- Adds `sqlx` (and a Postgres client) as a new optional dependency tree in the runtime crate

---

## Follow-ups

- [ ] Add `BackendKind::Postgres` to `api/types/backend_kind.rs` (this crate), `snake_case` deserialization (`"postgres"`)
- [ ] Add `url` + `queue_name` fields to `MessageBrokerConfig` for the `Postgres` variant
- [ ] Add `postgres` feature flag + `sqlx` optional dependency in `swe-edge-runtime-message-broker`
- [ ] Implement `PostgresMessageBroker` in `spi/broker/postgres/pgmq_message_broker.rs`
- [ ] Add `MessageBrokerFactory::postgres(dsn, queue_name)` in `saf/`
- [ ] Integration test `postgres_message_broker_int_test.rs` against a live Postgres with `pgmq` installed
- [ ] Document the `CREATE EXTENSION pgmq;` prerequisite and queue-vs-broadcast semantics difference
- [ ] Add a `postgres` backend example to `config/application.toml`

See sweengineeringlabs/edge-message-broker#5 for the tracked task list and acceptance criteria.

---

**Date Created:** 2026-09-05
**Last Updated:** 2026-09-05
