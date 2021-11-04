use std::{
	thread,
	time::{Duration, Instant},
};

use crate::ecs::system::InfallibleSystem;

pub struct Timer {
	instant: Option<Instant>,
	sleep: Option<Duration>,
}

impl Timer {
	pub fn sleep(&mut self, sleep: Duration) {
		self.sleep = Some(sleep);
	}
}

const DEFAULT_DELAY: Duration = Duration::from_millis(10);

impl<'ctx> InfallibleSystem<'ctx> for Timer {
	type Dependencies = ();

	fn create(_: Self::Dependencies) -> Self {
		Self {
			instant: None,
			sleep: None,
		}
	}

	fn update(&mut self, _: Self::Dependencies) {
		let instant = if let Some(prev) = self.instant {
			prev
		} else {
			self.instant = Some(Instant::now());

			return;
		};

		let mut time_left = DEFAULT_DELAY;

		if let Some(dur) = self.sleep {
			let time_spent = instant.elapsed();

			if dur > time_spent {
				time_left = dur - time_spent;
			}

			self.sleep = None;
		}

		while time_left > Duration::from_millis(0) {
			// TODO: yield from the system in order to process input and sound

			let delta = if time_left > DEFAULT_DELAY { DEFAULT_DELAY } else { time_left };
			thread::sleep(delta);

			time_left -= delta;
		}

		self.instant = Some(Instant::now());
	}
}
