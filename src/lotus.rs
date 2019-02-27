use crate::app::{Application, State, Command, Batch};
use crate::app::input::KeyCode;
use crate::data::Archive;
use crate::graphics::{Sprite, SpriteFont, Point, Size, SCREEN_START};
use crate::graphics::font::{Font, CHAR_SET_03, CHAR_SET_04};

#[macro_use] mod intro;
#[macro_use] mod menu;

const HELMET_SIZE: Size = Size { width: 48, height: 40 }; // 24 helmets of 48x40 bytes each
const SPLASH_SIZE: Size = Size { width: 16, height: 8 };
const MENU_ITEM_SIZE: Size = Size { width: 104, height: 26 };

macro_rules! yield_if {
	($ex:expr, $cmd:expr) => {
		if $ex {
			yield $cmd;
		}
	}
}

macro_rules! fade_out {
	() => {
		yield Batch::from(vec![
			Command::FadeOut,
			Command::Clear,
		]);
	}
}

#[derive(PartialEq)]
pub enum GameState {
	ProtectionScreen,
	Intro,
	MainMenu,
	Exit,
}

pub struct Lotus {
	state: GameState,
	archive: Archive,
	font_c03: Font,
	font_c04: Font,
	font_q1a: SpriteFont,
	input: String,
}

impl Lotus {
	pub fn new(mut archive: Archive) -> Self {
		let font_c03 = Font::from(CHAR_SET_03, archive.get("C03").unwrap());
		let font_c04 = Font::from(CHAR_SET_04, archive.get("C04").unwrap());
		let font_q1a = SpriteFont::from(archive.get("Q1A").unwrap());

		Self {
			state: GameState::ProtectionScreen,
			archive,
			font_c03,
			font_c04,
			font_q1a,
			input: String::new(),
		}
	}
}

impl Application for Lotus {
	fn start<'a>(&mut self) -> State<'a> {
		let app = self as *mut Lotus;

		Box::pin(move || {
			let state = unsafe { &mut (*app).state };
			let arc = unsafe { &mut (*app).archive };
			let font_c03 = unsafe { &(*app).font_c03 };
			let font_c04 = unsafe { &(*app).font_c04 };
			let font_q1a = unsafe { &(*app).font_q1a };
			let input = unsafe { &((*app).input) };

			if *state == GameState::ProtectionScreen {
				protection_screen!(state, arc, font_c03, input);
				fade_out!();
			}

			if *state == GameState::Intro {
				show_gremlin!(state, arc);
				fade_out!();
			}

			if *state == GameState::Intro {
				show_magnetic_fields!(state, arc);
				fade_out!();
			}

			if *state == GameState::Intro {
				show_credits!(state, arc, font_q1a);
				fade_out!();
			}

			*state = GameState::MainMenu;

			main_menu!(state, arc, font_c04);

			fade_out!();
		})
	}

	fn stop(&mut self) {
		self.state = GameState::Exit;
	}

	fn key_down(&mut self, key: KeyCode) {
		match self.state {
			GameState::ProtectionScreen => {
				match key.to_char() {
					Some(c) => {
						if self.input.len() < 3 {
							self.input.push(c);
						}
					},
					None => {
						match key {
							KeyCode::Return => { self.state = GameState::Intro; }
							KeyCode::Backspace => { self.input.pop(); }
							_ => {}
						}
					}
				}
			}
			GameState::Intro => {
				self.state = GameState::MainMenu;
			}
			GameState::MainMenu => {
				match key {
					KeyCode::Escape => { self.state = GameState::Exit; }
					_ => {}
				}
			}
			GameState::Exit => {}
		}
	}
}
