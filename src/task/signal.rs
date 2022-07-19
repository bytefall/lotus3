use std::{
	sync::{Arc, Condvar, Mutex},
	task::Wake,
};

enum SignalState {
	Empty,
	Waiting,
	Notified,
}

pub struct Signal {
	state: Mutex<SignalState>,
	cond: Condvar,
}

impl Signal {
	pub fn new() -> Self {
		Self {
			state: Mutex::new(SignalState::Empty),
			cond: Condvar::new(),
		}
	}

	pub fn wait(&self) {
		let mut state = self.state.lock().unwrap();

		match *state {
			SignalState::Notified => {
				// Notify() was called before we got here, consume it here without waiting and return immediately.
				*state = SignalState::Empty;
			}
			// This should not be possible because our signal is created within a function and never handed out to any
			// other threads. If this is the case, we have a serious problem so we panic immediately to avoid anything
			// more problematic happening.
			SignalState::Waiting => {
				unreachable!("Multiple threads waiting on the same signal: Open a bug report!");
			}
			SignalState::Empty => {
				// Nothing has happened yet, and we're the only thread waiting (as should be the case!). Set the state
				// accordingly and begin polling the condvar in a loop until it's no longer telling us to wait. The
				// loop prevents incorrect spurious wakeups.
				*state = SignalState::Waiting;

				while let SignalState::Waiting = *state {
					state = self.cond.wait(state).unwrap();
				}
			}
		}
	}

	pub fn notify(&self) {
		let mut state = self.state.lock().unwrap();

		match *state {
			// The signal was already notified, no need to do anything because the thread will be waking up anyway
			SignalState::Notified => {}
			// The signal wasnt notified but a thread isnt waiting on it, so we can avoid doing unnecessary work by
			// skipping the condvar and leaving behind a message telling the thread that a notification has already
			// occurred should it come along in the future.
			SignalState::Empty => *state = SignalState::Notified,
			// The signal wasnt notified and there's a waiting thread. Reset the signal so it can be wait()'ed on again
			// and wake up the thread. Because there should only be a single thread waiting, `notify_all` would also be
			// valid.
			SignalState::Waiting => {
				*state = SignalState::Empty;

				self.cond.notify_one();
			}
		}
	}
}

impl Wake for Signal {
	fn wake(self: Arc<Self>) {
		self.notify();
	}
}
