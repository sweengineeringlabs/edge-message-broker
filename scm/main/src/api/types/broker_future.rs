//! [`BrokerFuture`] — local wrapper around a boxed future, so `api/` never
//! names the foreign `futures::future::BoxFuture` type directly (per
//! `no_foreign_type`). Its `Future` impl and constructor live in `core/`
//! (`api_no_inherent_impl` — `api/` is a declaration-only layer).

use std::future::Future;
use std::pin::Pin;

/// A boxed, pinned, `Send` future — the async return type for
/// [`crate::MessageBroker`] operations.
pub struct BrokerFuture<'a, T>(pub(crate) Pin<Box<dyn Future<Output = T> + Send + 'a>>);
