use anyhow::Result;
use std::{
	cell::RefCell,
	future::Future,
	pin::Pin,
	rc::Rc,
	task::{Context, Poll},
};
use winput::WinitInputHelper;

use crate::{
	data::Archive,
	game::options::Config,
	screen::Screen,
	systems::{System, SystemEnum, Timer},
	task::Signal,
};

pub struct State {
	pub arc: Archive,
	pub cfg: Config,
	pub input: Rc<RefCell<WinitInputHelper>>,
	pub screen: Screen,
}

pub struct GameEngine {
	systems: [SystemEnum; 1],
	task: Pin<Box<dyn Future<Output = Result<()>>>>,
}

impl GameEngine {
	pub fn new<T: Future<Output = Result<()>> + 'static>(
		arc: Archive,
		cfg: Config,
		input: Rc<RefCell<WinitInputHelper>>,
		screen: Screen,
		f: fn(State) -> T,
	) -> Result<Self> {
		let state = State {
			arc,
			cfg,
			input,
			screen,
		};

		Ok(Self {
			systems: [Timer::default().into()],
			task: Box::pin(f(state)),
		})
	}

	pub fn step(&mut self, ctx: &mut Context<'static>, signal: &Signal) -> Result<Poll<()>> {
		for s in &mut self.systems {
			s.update()?;
		}

		Ok(match self.task.as_mut().poll(ctx) {
			Poll::Pending => {
				signal.wait();

				Poll::Pending
			}
			Poll::Ready(result) => Poll::Ready(result?),
		})
	}
}
