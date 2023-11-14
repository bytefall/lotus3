use anyhow::Result;
use std::{
	cell::RefCell,
	future::Future,
	pin::Pin,
	rc::Rc,
	task::{Context, Poll},
};

use crate::{data::Archive, game::options::Config, input::InputHelper, task::Signal};

pub struct State {
	pub arc: Archive,
	pub cfg: Config,
	pub input: Rc<RefCell<InputHelper>>,
}

pub struct GameEngine {
	task: Pin<Box<dyn Future<Output = Result<()>>>>,
}

impl GameEngine {
	pub fn new<T: Future<Output = Result<()>> + 'static>(
		arc: Archive,
		cfg: Config,
		input: Rc<RefCell<InputHelper>>,
		f: fn(State) -> T,
	) -> Result<Self> {
		let state = State { arc, cfg, input };

		Ok(Self {
			task: Box::pin(f(state)),
		})
	}

	pub fn step(&mut self, ctx: &mut Context<'static>, signal: &Signal) -> Result<Poll<()>> {
		Ok(match self.task.as_mut().poll(ctx) {
			Poll::Pending => {
				signal.wait();

				Poll::Pending
			}
			Poll::Ready(result) => Poll::Ready(result?),
		})
	}
}
