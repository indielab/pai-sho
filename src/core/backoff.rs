//! Reconnect delays. Pure: it holds no clock and does no sleeping.
//!
//! The shell in `peer.rs` asks for the next delay, sleeps it, dials, and
//! reports what happened. Keeping the schedule here means the climb to the
//! ceiling is a unit test rather than a stopwatch held to a live daemon.

use std::time::Duration;

/// First delay after a failed dial
pub const INITIAL: Duration = Duration::from_secs(1);
/// Ceiling the delay climbs to and stays at
pub const MAX: Duration = Duration::from_secs(60);

pub struct Backoff {
    next: Duration,
}

impl Backoff {
    /// A peer we have never reached. Its first dial is immediate: the delay
    /// only starts applying once an attempt has failed.
    pub fn unreached() -> Self {
        Self {
            next: Duration::ZERO,
        }
    }

    /// A peer whose connection just dropped. It waits one step before redialing.
    pub fn connected() -> Self {
        Self { next: INITIAL }
    }

    /// How long to wait before the next dial
    pub fn delay(&self) -> Duration {
        self.next
    }

    /// The dial failed. Double the delay, up to the ceiling.
    pub fn failed(&mut self) {
        self.next = (self.next * 2).max(INITIAL).min(MAX);
    }

    /// We are connected again. Back to the first delay.
    pub fn reset(&mut self) {
        self.next = INITIAL;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_dial_is_immediate() {
        assert_eq!(Backoff::unreached().delay(), Duration::ZERO);
    }

    #[test]
    fn a_dropped_connection_waits_a_step() {
        assert_eq!(Backoff::connected().delay(), INITIAL);
    }

    /// A peer whose daemon is not up yet must not be dialed in a tight loop.
    /// Its delay starts at zero, and zero doubled is still zero.
    #[test]
    fn an_unreached_peer_backs_off() {
        let mut backoff = Backoff::unreached();
        backoff.failed();
        assert_eq!(backoff.delay(), INITIAL);

        let mut seen = backoff.delay();
        for _ in 0..10 {
            backoff.failed();
            assert!(
                backoff.delay() > seen || backoff.delay() == MAX,
                "delay stalled at {:?}",
                backoff.delay()
            );
            seen = backoff.delay();
        }
        assert_eq!(seen, MAX);
    }

    #[test]
    fn the_delay_climbs_to_the_ceiling_and_stays() {
        let mut backoff = Backoff::connected();
        for expected in [2, 4, 8, 16, 32, 60, 60, 60] {
            backoff.failed();
            assert_eq!(backoff.delay(), Duration::from_secs(expected));
        }
    }

    #[test]
    fn connecting_again_clears_the_climb() {
        let mut backoff = Backoff::connected();
        for _ in 0..8 {
            backoff.failed();
        }
        assert_eq!(backoff.delay(), MAX);
        backoff.reset();
        assert_eq!(backoff.delay(), INITIAL);
    }
}
