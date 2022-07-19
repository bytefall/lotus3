use std::{
	future::Future,
	pin::Pin,
	task::{Context, Poll},
	time::{Duration, Instant},
};

pub struct Timer<'a> {
	instant: Instant,
	duration: Duration,
	cancel: Option<&'a dyn Fn() -> bool>,
}

impl<'a> Timer<'a> {
	pub fn new(duration: Duration) -> Self {
		Self {
			instant: Instant::now(),
			duration,
			cancel: None,
		}
	}

	pub fn with_cancel(mut self, cancel: &'a dyn Fn() -> bool) -> Self {
		self.cancel = Some(cancel);
		self
	}
}

impl Future for Timer<'_> {
	type Output = bool;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let time_spent = self.instant.elapsed();

		if time_spent >= self.duration {
			Poll::Ready(false)
		} else {
			if time_spent.as_millis() > 0 && self.cancel.as_ref().map_or(false, |f| f()) {
				return Poll::Ready(true);
			}

			cx.waker().wake_by_ref();

			Poll::Pending
		}
	}
}
