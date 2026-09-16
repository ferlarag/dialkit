//! Bounded, operation-aware retry policy.

use std::{num::NonZeroU32, time::Duration};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationSafety {
    Read,
    Mutation,
}

#[derive(Clone, Debug)]
pub struct RetryPolicy {
    max_attempts: NonZeroU32,
    initial_delay: Duration,
    max_delay: Duration,
    retry_mutations: bool,
}

impl RetryPolicy {
    #[must_use]
    pub fn conservative() -> Self {
        Self {
            max_attempts: NonZeroU32::new(3).expect("three is nonzero"),
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(2),
            retry_mutations: false,
        }
    }

    #[must_use]
    pub const fn max_attempts(mut self, attempts: NonZeroU32) -> Self {
        self.max_attempts = attempts;
        self
    }

    #[must_use]
    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay.min(self.max_delay);
        self
    }

    #[must_use]
    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay.max(Duration::from_millis(1));
        self.initial_delay = self.initial_delay.min(self.max_delay);
        self
    }

    /// Opts into retrying ambiguous mutation failures, which can duplicate side effects.
    #[must_use]
    pub const fn retry_mutations(mut self, enabled: bool) -> Self {
        self.retry_mutations = enabled;
        self
    }

    #[must_use]
    pub const fn attempts(&self) -> u32 {
        self.max_attempts.get()
    }

    #[must_use]
    pub const fn retries_ambiguous(&self, safety: OperationSafety) -> bool {
        matches!(safety, OperationSafety::Read) || self.retry_mutations
    }

    #[must_use]
    pub fn delay(&self, attempt: u32, retry_after: Option<Duration>) -> Duration {
        if let Some(value) = retry_after {
            return value.min(self.max_delay);
        }
        let factor = 2_u32.saturating_pow(attempt.saturating_sub(1));
        let ceiling = self
            .initial_delay
            .saturating_mul(factor)
            .min(self.max_delay);
        ceiling.mul_f64(rand::random::<f64>())
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::conservative()
    }
}
