use std::ops::Generator;
use std::pin::Pin;
use std::time::Duration;

mod sdl;
pub use self::sdl::run;

pub mod input;
use self::input::KeyCode;

use crate::graphics::{Drawable, Printable, Point}; // FIXME: use 'core' module

pub struct Config {
	pub title: &'static str,
	pub width: usize,
	pub height: usize,
}

pub enum Command<'a> {
	Palette(Vec<u8>),
	Draw(Box<Drawable>, Point),
	Print(&'a Printable, &'static str, Point),
	Pop,
	Clear,
	Present,
	FadeIn,
	FadeOut,
}

pub struct Batch<'a> {
	pub commands: Vec<Command<'a>>,
	pub sleep: Option<Duration>,
}

impl<'a> Batch<'a> {
	pub fn from(commands: Vec<Command<'a>>) -> Self {
		Self {
			commands,
			sleep: None,
		}
	}

	pub fn sleep(mut self, ms: u16) -> Self {
		self.sleep = Some(Duration::from_millis(ms.into()));
		self
	}
}

pub type State<'a> = Pin<Box<dyn Generator<Yield = Batch<'a>, Return = ()> + Unpin>>;

pub trait Application {
	fn start<'a>(&mut self) -> State<'a>;

	fn stop(&mut self);

	fn key_down(&mut self, key: KeyCode);
}
