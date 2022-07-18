use eyre::Result;
use std::{
	cell::RefCell,
	rc::Rc,
	sync::Arc,
	task::{Context, Poll, Waker},
};
use winit::{
	dpi::{LogicalPosition, PhysicalSize},
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Window as WinitWindow, WindowBuilder},
};
use winput::WinitInputHelper;

use crate::{
	graphics::{SCREEN_HEIGHT, SCREEN_WIDTH},
	screen::Screen,
	task::Signal,
};

const SCREEN_SCALE: u32 = 2;

pub struct Application {
	window: WinitWindow,
	event_loop: EventLoop<()>,
	input: Rc<RefCell<WinitInputHelper>>,
}

impl Application {
	pub fn new(title: &str) -> Result<Self> {
		let size = PhysicalSize::new(SCREEN_WIDTH * SCREEN_SCALE, SCREEN_HEIGHT * SCREEN_SCALE);
		let event_loop = EventLoop::new();

		let window = WindowBuilder::new()
			.with_visible(false)
			.with_title(title)
			.with_inner_size(size)
			.with_min_inner_size(size)
			.with_resizable(false)
			.build(&event_loop)?;

		let (monitor_width, monitor_height) = {
			if let Some(monitor) = window.current_monitor() {
				let size = monitor.size().to_logical(window.scale_factor());
				(size.width, size.height)
			} else {
				(SCREEN_WIDTH, SCREEN_HEIGHT)
			}
		};

		let center = LogicalPosition::new(
			(monitor_width - SCREEN_WIDTH * SCREEN_SCALE) as f32 / 2.0,
			(monitor_height - SCREEN_HEIGHT * SCREEN_SCALE) as f32 / 2.0,
		);

		window.set_outer_position(center);
		window.set_visible(true);

		Ok(Self {
			window,
			event_loop,
			input: Rc::new(RefCell::new(WinitInputHelper::new())),
		})
	}

	pub fn get_screen(&self) -> Result<Screen> {
		Screen::from_window(&self.window)
	}

	pub fn get_input(&self) -> Rc<RefCell<WinitInputHelper>> {
		Rc::clone(&self.input)
	}

	pub fn run(self, mut step: impl FnMut(&mut Context<'static>, &Signal) -> Result<Poll<()>> + 'static) -> ! {
		let signal = Arc::new(Signal::new());
		let waker = Waker::from(Arc::clone(&signal));
		let mut ctx = Context::from_waker(Box::leak(Box::new(waker)));

		self.event_loop.run(move |event, _, control_flow| {
			match &event {
				Event::WindowEvent { event, .. } if event == &WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit;
					return;
				}
				Event::LoopDestroyed => {
					return;
				}
				_ => {
					if self.input.borrow_mut().update(&event) {
						let result = step(&mut ctx, &signal);

						match result {
							Ok(Poll::Pending) => (),
							Ok(Poll::Ready(())) => *control_flow = ControlFlow::Exit,
							Err(err) => {
								eprintln!("{:?}", err);

								*control_flow = ControlFlow::Exit;
							}
						}
					}
				}
			};
		});
	}
}
