use std::{
    future::Future,
    time::{Duration, Instant},
};

#[allow(dead_code)]
pub struct ExpiringLazy<T, F, Fut>
where
    F: Fn() -> Fut,
    Fut: Future<Output = T>,
{
    value: T,
    init: F,
    expires_at: Instant,
    ttl: Duration,
}

#[allow(dead_code)]
impl<T, F, Fut> ExpiringLazy<T, F, Fut>
where
    F: Fn() -> Fut,
    Fut: Future<Output = T>,
{
    pub async fn new(ttl: Duration, init: F) -> Self {
        Self {
            value: init().await,
            init,
            expires_at: Instant::now() + ttl,
            ttl,
        }
    }

    pub async fn get(&mut self) -> &T {
        let now = Instant::now();

        if now > self.expires_at {
            let f = &self.init;
            self.value = f().await;
            self.expires_at = now + self.ttl;
        }

        &self.value
    }
}
