use crate::{
	game::state::GameState,
	graphics::{PaintFn, Point, Size, Sprite},
};
use std::{collections::VecDeque, time::Duration};

pub enum Layer {
	Back,
	Front,
}

pub const ALL: Option<Layer> = None;
pub const BACK: Option<Layer> = Some(Layer::Back);
pub const FRONT: Option<Layer> = Some(Layer::Front);

pub enum Command {
	Palette(Vec<u8>),
	Draw(Option<Layer>, Sprite, Point),
	Paint(Option<Layer>, Box<PaintFn>, Size, Point),
	Print(Option<Layer>, &'static str, Point),
	Present,
	Clear(Option<Layer>),
	FadeIn(Option<Layer>),
	FadeOut(Option<Layer>),
	FadeOutByColorIndex(usize),
	State(GameState),
}

pub struct CommandBatch {
	pub commands: Vec<Command>,
	pub sleep: Option<Duration>,
}

impl CommandBatch {
	pub fn palette(&mut self, pal: Vec<u8>) -> &mut Self {
		self.commands.push(Command::Palette(pal));
		self
	}

	pub fn draw(&mut self, target: Option<Layer>, sprite: Sprite, pos: Point) -> &mut Self {
		self.commands.push(Command::Draw(target, sprite, pos));
		self
	}

	pub fn paint(&mut self, target: Option<Layer>, foo: Box<PaintFn>, size: Size, pos: Point) -> &mut Self {
		self.commands.push(Command::Paint(target, foo, size, pos));
		self
	}

	pub fn print(&mut self, target: Option<Layer>, text: &'static str, pos: Point) -> &mut Self {
		self.commands.push(Command::Print(target, text, pos));
		self
	}

	pub fn present(&mut self) -> &mut Self {
		self.commands.push(Command::Present);
		self
	}

	pub fn clear(&mut self, target: Option<Layer>) -> &mut Self {
		self.commands.push(Command::Clear(target));
		self
	}

	pub fn fade_in(&mut self, target: Option<Layer>) -> &mut Self {
		self.commands.push(Command::FadeIn(target));
		self
	}

	pub fn fade_out(&mut self, target: Option<Layer>) -> &mut Self {
		self.commands.push(Command::FadeOut(target));
		self
	}

	pub fn fade_out_color(&mut self, ix: usize) -> &mut Self {
		self.commands.push(Command::FadeOutByColorIndex(ix));
		self
	}

	pub fn state(&mut self, state: GameState) -> &mut Self {
		self.commands.push(Command::State(state));
		self
	}
}

pub struct CommandSequence(VecDeque<CommandBatch>);

impl CommandSequence {
	pub fn new() -> Self {
		CommandSequence(VecDeque::new())
	}

	pub fn batch(&mut self, timeout: Option<u16>) -> &mut CommandBatch {
		self.0.push_back(CommandBatch {
			commands: Vec::new(),
			sleep: timeout.map(|ms| Duration::from_millis(ms.into())),
		});
		self.0.back_mut().unwrap()
	}

	pub fn pop(&mut self) -> Option<CommandBatch> {
		self.0.pop_front()
	}

	pub fn clear(&mut self) {
		self.0.clear();
	}
}
