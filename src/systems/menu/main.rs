use eyre::Result;
use winit::event::VirtualKeyCode;
use winit_input_helper::TextChar;

use super::{
	START_ITEM_POS, DEFINE_ITEM_POS,
	FRAME_OFFSET, FRAME_SIZE_ST,
	MENU_ITEM_SIZE,
};
use crate::{
	data::Archive,
	ecs::{
		context::ControlFlow,
		system::System,
	},
	game::{
		options::{Config, Acceleration, Course, Transmission, Race},
		state::{GameState, GameFlow, Screen},
	},
	graphics::{Drawable as _, Sprite, SCREEN_START, font::Font, Frame, FRAME_BORDER},
	systems::{Cache, Window},
};

macro_rules! switch_option {
	($opt:expr, $v1:expr, $v2:expr) => {
		$opt = if $opt == $v1 { $v2 } else { $v1 };
	};
}

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		arc: &'ctx Archive,
		cache: &'ctx Cache,
		cfg: &'ctx mut Config,
		ctrl: &'ctx mut ControlFlow,
		flow: &'ctx mut GameFlow,
		win: &'ctx mut Window,
	}
}

struct Assets {
	bgr: Sprite,
	trans: [Sprite; 2],
	accel: [Sprite; 2],
	race: [Sprite; 2],
	player: [Sprite; 2],
	course: [Sprite; 5],
}

#[derive(Default)]
pub struct Menu {
	assets: Option<Assets>,
}

impl<'ctx> System<'ctx> for Menu {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self::default())
	}

	fn update(&mut self, dep: Self::Dependencies) -> Result<()> {
		let (row, col, editor) = if let GameState::Menu(Screen::Main { row, col, editor }) = &mut dep.flow.state {
			(row, col, editor)
		} else {
			return Ok(());
		};

		if dep.flow.changed {
			dep.flow.changed = false;

			let (pal, a) = load(dep.arc)?;
			self.assets = Some(a);
			dep.win.palette = pal;

			render(dep.win, self.assets.as_ref().unwrap(), dep.cfg, &dep.cache.font_c03, &dep.cache.font_c04, *row, *col);

			dep.win.fade_in();
		}

		let prev_cfg = dep.cfg.clone();
		let prev_state = (*row, *col, *editor);
		let mut quit = false;
		let mut new_state = None;

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Up) && *row > 0 {
			*row -= 1;
			*editor = false;
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Down) && *row < 4 {
			*row += 1;
			*editor = false;
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Left) && *col > 0 {
			*col -= 1;
			*editor = false;
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Right) && *col < 2 {
			*col += 1;
			*editor = false;
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Escape) {
			if *editor {
				*editor = false;
			} else {
				quit = true;
			}
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Return) {
			match (*row, *col) {
				(0, 0) => {
					*editor = !*editor;
				}
				START_ITEM_POS => {
					new_state = Some(GameState::ModelSelect);
				}
				(0, 2) => {
					*editor = !*editor;
				}
				(1, 0) => {
					switch_option!(dep.cfg.p1_trans, Transmission::Manual, Transmission::Automatic);
				}
				(1, 1) => {
					switch_option!(dep.cfg.race, Race::TimeLimit, Race::Competition);
				}
				(1, 2) => {
					switch_option!(dep.cfg.p2_trans, Transmission::Manual, Transmission::Automatic);
				}
				(2, 0) => {
					switch_option!(dep.cfg.p1_accel, Acceleration::Button, Acceleration::Joystick);
				}
				(2, 1) => {
					dep.cfg.course = match dep.cfg.course {
						Course::T1 => Course::T2,
						Course::T2 => Course::T3,
						Course::T3 => Course::Circular,
						Course::Circular => Course::Unknown,
						Course::Unknown => Course::T1,
					};
				}
				(2, 2) => {
					switch_option!(dep.cfg.p2_accel, Acceleration::Button, Acceleration::Joystick);
				}
				(3, 0) => {
					// Controls
				}
				(3, 1) => {
					switch_option!(dep.cfg.players_num, 1, 2);
				}
				(3, 2) => {
					// Sound Settings
				}
				(4, 0) => {
					// RECS
				}
				(4, 1) => {
					*editor = !*editor;
				}
				DEFINE_ITEM_POS => {
					new_state = Some(GameState::define_menu());
				}
				_ => {}
			}
		}

		if *editor {
			for key in dep.ctrl.input.text() {
				match key {
					TextChar::Char(c) => match (*row, *col) {
						(0, 0) if dep.cfg.p1_name.len() < 12 => dep.cfg.p1_name.push(c),
						(0, 2) if dep.cfg.p2_name.len() < 12 => dep.cfg.p2_name.push(c),
						(4, 1) if dep.cfg.code.len() < 12 => dep.cfg.code.push(c),
						_ => {}
					}
					TextChar::Back => match (*row, *col) {
						(0, 0) => { dep.cfg.p1_name.pop(); }
						(0, 2) => { dep.cfg.p2_name.pop(); }
						(4, 1) => { dep.cfg.code.pop(); }
						_ => {}
					}
				}
			}
		}

		if quit {
			dep.ctrl.quit_requested = true;

			return Ok(());
		}

		if let Some(state) = new_state {
			self.assets = None;

			dep.flow.set(state);
			dep.win.fade_out();
			dep.ctrl.input = winit_input_helper::WinitInputHelper::new();

			return Ok(());
		}

		if (*row, *col, *editor) != prev_state || dep.cfg != &prev_cfg {
			render(dep.win, self.assets.as_ref().unwrap(), dep.cfg, &dep.cache.font_c03, &dep.cache.font_c04, *row, *col);
			dep.win.present();
		}

		Ok(())
	}
}

fn load(arc: &Archive) -> Result<(Vec<u8>, Assets)> {
	let (i14, pal) = arc.get_with_palette("I14")?;
	let i15 = arc.get_series("I15", MENU_ITEM_SIZE.width * MENU_ITEM_SIZE.height)?;

	let a = Assets {
		bgr: Sprite::from(i14),
		trans: [
			Sprite::from(i15.get(0).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Transmission::Manual
			Sprite::from(i15.get(1).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Transmission::Automatic
		],
		accel: [
			Sprite::from(i15.get(2).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Acceleration::Button
			Sprite::from(i15.get(3).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Acceleration::Joystick
		],
		race: [
			Sprite::from(i15.get(6).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Race::TimeLimit
			Sprite::from(i15.get(7).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Race::Competition
		],
		player: [
			Sprite::from(i15.get(8).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // 1 player
			Sprite::from(i15.get(9).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // 2 players
		],
		course: [
			Sprite::from(i15.get(10).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Course::T1
			Sprite::from(i15.get(11).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Course::T2
			Sprite::from(i15.get(12).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Course::T3
			Sprite::from(i15.get(13).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Course::Circular
			Sprite::from(i15.get(14).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Course::Unknown
		],
	};

	Ok((pal, a))
}

fn render(win: &mut Window, a: &Assets, cfg: &Config, font_c03: &Font, font_c04: &Font, row: u8, col: u8) {
	win.draw(&a.bgr, SCREEN_START);

	win.draw(&a.trans[cfg.p1_trans as usize], (6, 52).into());
	win.draw(&a.accel[cfg.p1_accel as usize], (6, 91).into());
	win.draw(&a.trans[cfg.p2_trans as usize], (214, 52).into());
	win.draw(&a.accel[cfg.p2_accel as usize], (214, 91).into());
	win.draw(&a.race[cfg.race as usize], (110, 52).into());
	win.draw(&a.course[cfg.course as usize], (110, 91).into());
	win.draw(&a.player[cfg.players_num as usize - 1], (110, 130).into());

	win.print(&cfg.p1_name, font_c04, (13, 21).into());
	win.print(&cfg.p2_name, font_c04, (221, 21).into());
	win.print(&cfg.code, font_c03, (117, 177).into());

	let frame = Frame::new(FRAME_SIZE_ST);
	let pos = (
		(col as u32 * (frame.width() - FRAME_BORDER + 1) + FRAME_OFFSET.0),
		(row as u32 * (frame.height() - FRAME_BORDER + 1) + FRAME_OFFSET.1),
	).into();

	win.draw(&frame, pos);
}
