use std::time::Duration;

mod oneshot;
mod signal;
mod timer;

use oneshot::Oneshot;
use timer::Timer;

pub use signal::Signal;

pub fn yield_now() -> impl std::future::Future<Output = ()> {
    Oneshot::default()
}

pub fn sleep(ms: u64) -> Timer<'static> {
    Timer::new(Duration::from_millis(ms))
}
