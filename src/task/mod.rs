use std::{cell::RefCell, rc::Rc, time::Duration};

mod oneshot;
mod signal;
mod timer;

use oneshot::Oneshot;
use timer::Timer;

pub use signal::Signal;

pub struct CancellationToken {
	is_cancelled: bool,
}

impl CancellationToken {
	pub fn new() -> Self {
		Self { is_cancelled: false }
	}

	pub fn cancel(&mut self) {
		self.is_cancelled = true;
	}

	pub fn clear(&mut self) {
		self.is_cancelled = false;
	}

	pub fn cancelled(&self) -> bool {
		self.is_cancelled
	}
}

pub type CancellationTokenType = Rc<RefCell<CancellationToken>>;

pub fn yield_now() -> impl std::future::Future<Output = ()> {
	Oneshot::default()
}

pub fn sleep(ms: u64) -> Timer {
	Timer::new(Duration::from_millis(ms))
}
