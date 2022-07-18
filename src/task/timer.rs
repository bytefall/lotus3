use std::{
	future::Future,
	pin::Pin,
	rc::Rc,
	task::{Context, Poll},
	time::{Duration, Instant},
};

use super::CancellationTokenType;

pub struct Timer {
	instant: Instant,
	duration: Duration,
	token: Option<CancellationTokenType>,
}

impl Timer {
	pub fn new(duration: Duration) -> Self {
		Self {
			instant: Instant::now(),
			duration,
			token: None,
		}
	}

	pub fn with_token(mut self, token: &CancellationTokenType) -> Self {
		self.token = Some(Rc::clone(token));
		self
	}
}

impl Future for Timer {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let time_spent = self.instant.elapsed();

		if time_spent >= self.duration {
			Poll::Ready(())
		} else {
			if time_spent.as_millis() > 0 && self.token.as_ref().map_or(false, |t| t.borrow().cancelled()) {
				return Poll::Ready(());
			}

			cx.waker().wake_by_ref();

			Poll::Pending
		}
	}
}
