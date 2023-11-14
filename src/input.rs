use winit::{
	event::{ElementState, KeyEvent},
	keyboard::{Key, NamedKey},
	platform::modifier_supplement::KeyEventExtModifierSupplement,
};

pub const ENTER_CHAR: char = '\r';
pub const BACKSPACE_CHAR: char = '\x08';
pub const ESCAPE_CHAR: char = '\x1b';

pub struct InputHelper {
	keys: Vec<Key>,
}

impl InputHelper {
	pub fn new() -> Self {
		Self { keys: Vec::new() }
	}

	pub fn handle(&mut self, event: KeyEvent) {
		if event.state != ElementState::Pressed {
			return;
		}

		self.keys.push(event.key_without_modifiers());
	}

	pub fn clear(&mut self) {
		self.keys.clear();
	}

	pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
		self.keys.iter().filter_map(|x| x.to_text()).flat_map(|x| x.chars())
	}

	pub fn keys(&self) -> impl Iterator<Item = &Key> {
		self.keys.iter()
	}

	pub fn key_pressed(&self, key: NamedKey) -> bool {
		self.keys.contains(&Key::Named(key))
	}
}
