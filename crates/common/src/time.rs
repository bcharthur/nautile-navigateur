use std::time::Instant;

pub fn now() -> Instant { Instant::now() }

pub fn millis_since(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
