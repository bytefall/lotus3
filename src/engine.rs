use eyre::{ErrReport, Result};
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
	task::{CancellationToken, Signal},
};

type OnCancel = fn(&WinitInputHelper) -> bool;

pub struct State {
	pub arc: Archive,
	pub cfg: Config,
	pub input: Rc<RefCell<WinitInputHelper>>,
	pub screen: Screen,
	pub on_cancel: Rc<RefCell<Option<OnCancel>>>,
	pub token: Rc<RefCell<CancellationToken>>,
}

pub struct GameEngine {
	systems: [SystemEnum; 1],
	task: Pin<Box<dyn Future<Output = Result<(), ErrReport>>>>,
	on_cancel: Rc<RefCell<Option<OnCancel>>>,
	token: Rc<RefCell<CancellationToken>>,
	input: Rc<RefCell<WinitInputHelper>>,
}

impl GameEngine {
	pub fn new<T: Future<Output = Result<(), ErrReport>> + 'static>(
		arc: Archive,
		cfg: Config,
		input: Rc<RefCell<WinitInputHelper>>,
		screen: Screen,
		f: fn(State) -> T,
	) -> Result<Self> {
		let on_cancel = Rc::new(RefCell::new(None));
		let token = Rc::new(RefCell::new(CancellationToken::new()));

		let state = State {
			arc,
			cfg,
			input: Rc::clone(&input),
			screen,
			on_cancel: Rc::clone(&on_cancel),
			token: Rc::clone(&token),
		};

		Ok(Self {
			systems: [Timer::default().into()],
			task: Box::pin(f(state)),
			on_cancel,
			token,
			input,
		})
	}

	pub fn step(&mut self, ctx: &mut Context<'static>, signal: &Signal) -> Result<Poll<()>> {
		for s in &mut self.systems {
			s.update()?;
		}

		{
			let on_cancel = self.on_cancel.borrow();

			if let Some(f) = on_cancel.as_ref() {
				let input: &WinitInputHelper = &self.input.borrow();

				if f(input) {
					self.token.borrow_mut().cancel();
				}
			}
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
