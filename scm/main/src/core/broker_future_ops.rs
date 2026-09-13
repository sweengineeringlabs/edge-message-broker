//! [`BrokerFuture`]'s constructor and [`Future`] impl. Kept in `core/`, not
//! `api/` (`api_no_inherent_impl` — `api/` is a declaration-only layer).

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::api::BrokerFuture;

impl<'a, T> BrokerFuture<'a, T> {
    /// Wrap `fut` for use as a trait method's return type.
    pub fn new(fut: impl Future<Output = T> + Send + 'a) -> Self {
        Self(Box::pin(fut))
    }
}

impl<'a, T> Future for BrokerFuture<'a, T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        self.0.as_mut().poll(cx)
    }
}
