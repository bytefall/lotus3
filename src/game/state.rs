pub struct GameFlow {
	pub state: GameState,
	pub changed: bool,
}

impl GameFlow {
	pub fn new(state: GameState) -> Self {
		Self { state, changed: true }
	}

	pub fn current(&self) -> &GameState {
		&self.state
	}

	pub fn set(&mut self, state: GameState) {
		self.state = state;
		self.changed = true;
	}
}

#[derive(PartialEq)]
pub enum GameState {
	Protection(String),
	Intro,
	Menu(Screen),
	ModelSelect,
	AudioTuner,
}

impl GameState {
	pub fn main_menu() -> Self {
		Self::main_menu_pos((0, 1))
	}

	pub fn main_menu_pos(pos: (u8, u8)) -> Self {
		Self::Menu(Screen::Main {
			row: pos.0,
			col: pos.1,
			editor: false,
		})
	}

	pub fn define_menu() -> Self {
		Self::Menu(Screen::Define { row: 0, editor: false })
	}
}

#[derive(Clone, PartialEq)]
pub enum Screen {
	Main { row: u8, col: u8, editor: bool },
	Define { row: u8, editor: bool },
}
