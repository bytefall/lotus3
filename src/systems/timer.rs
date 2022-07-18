use eyre::Result;
use std::{
	thread,
	time::{Duration, Instant},
};

use super::System;

#[derive(Default)]
pub struct Timer {
	instant: Option<Instant>,
}

const DEFAULT_DELAY: Duration = Duration::from_millis(10);

impl System for Timer {
	fn update(&mut self) -> Result<()> {
		let instant = self.instant.get_or_insert(Instant::now());
		let elapsed = instant.elapsed();

		if elapsed < DEFAULT_DELAY {
			thread::sleep(DEFAULT_DELAY - elapsed);
		}

		*instant = Instant::now();

		Ok(())
	}
}
