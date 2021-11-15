use generational_arena::Index;
use eyre::Result;

use super::{
	build_frame,
	START_ITEM_POS, DEFINE_ITEM_POS,
	FRAME_BORDER, FRAME_OFFSET, FRAME_SIZE_ST,
	MENU_ITEM_SIZE,
};
use crate::{
	data::Archive,
	ecs::{
		context::ControlFlow,
		system::System,
	},
	game::{
		input::KeyCode,
		options::{Config, Acceleration, Course, Transmission, Race},
		state::{GameState, GameFlow, Screen},
	},
	graphics::{Sprite, SCREEN_START, font::Font},
	systems::{Cache, Input, Window},
};

macro_rules! switch_option {
	($opt:expr, $v1:expr, $v2:expr) => {
		$opt = if $opt == $v1 { $v2 } else { $v1 };
	}
}

struct Store {
	bg: Index,
	frame: Index,
	trans: [Index; 2],
	accel: [Index; 2],
	race: [Index; 2],
	player: [Index; 2],
	course: [Index; 5],
}

pub struct Menu {
	store: Option<Store>,
}

impl Menu {
	fn prepare(&mut self, win: &mut Window, arc: &Archive) -> Result<()> {
		let (i14, pal) = arc.get_with_palette("I14")?;
		let i15 = arc.get_series("I15", MENU_ITEM_SIZE.width * MENU_ITEM_SIZE.height)?;

		win.palette = pal;

		self.store = Some(Store {
			bg: win.draw(&Sprite::from(i14)).id,
			frame: win.paint(FRAME_SIZE_ST, |_, c| build_frame(FRAME_SIZE_ST, c)).id,
			trans: [
				win.draw(&Sprite::from(i15.get(0).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Transmission::Manual
				win.draw(&Sprite::from(i15.get(1).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Transmission::Automatic
			],
			accel: [
				win.draw(&Sprite::from(i15.get(2).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Acceleration::Button
				win.draw(&Sprite::from(i15.get(3).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Acceleration::Joystick
			],
			race: [
				win.draw(&Sprite::from(i15.get(6).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Race::TimeLimit
				win.draw(&Sprite::from(i15.get(7).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Race::Competition
			],
			player: [
				win.draw(&Sprite::from(i15.get(8).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // 1 player
				win.draw(&Sprite::from(i15.get(9).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // 2 players
			],
			course: [
				win.draw(&Sprite::from(i15.get(10).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Course::T1
				win.draw(&Sprite::from(i15.get(11).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Course::T2
				win.draw(&Sprite::from(i15.get(12).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Course::T3
				win.draw(&Sprite::from(i15.get(13).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Course::Circular
				win.draw(&Sprite::from(i15.get(14).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)).id, // Course::Unknown
			],
		});

		Ok(())
	}

	fn show(&self, win: &mut Window, cfg: &Config, font_c04: &Font, row: u8, col: u8) {
		let store = self.store.as_ref().unwrap();

		win.show(store.bg, SCREEN_START);

		win.show(*store.trans.get(cfg.p1_trans as usize).unwrap(), (6, 52).into());
		win.show(*store.accel.get(cfg.p1_accel as usize).unwrap(), (6, 91).into());
		win.show(*store.trans.get(cfg.p2_trans as usize).unwrap(), (214, 52).into());
		win.show(*store.accel.get(cfg.p2_accel as usize).unwrap(), (214, 91).into());
		win.show(*store.race.get(cfg.race as usize).unwrap(), (110, 52).into());
		win.show(*store.course.get(cfg.course as usize).unwrap(), (110, 91).into());
		win.show(*store.player.get(cfg.players_num as usize - 1).unwrap(), (110, 130).into());

		win.print(font_c04, &cfg.p1_name).show((13, 21).into());
		win.print(font_c04, &cfg.p2_name).show((221, 21).into());
		win.print(font_c04, &cfg.code).show((117, 177).into());

		// frame should be the last (i.e. on top of everything)
		let size = win.txt_size(store.frame).unwrap();
		let rect = (
			(FRAME_OFFSET.0 + col as u32 * (size.width - FRAME_BORDER)) as i32,
			(FRAME_OFFSET.1 + row as u32 * (size.height - FRAME_BORDER)) as i32,
		).into();

		win.show(store.frame, rect);
	}
}

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		arc: &'ctx Archive,
		cache: &'ctx Cache,
		cfg: &'ctx mut Config,
		ctrl: &'ctx mut ControlFlow,
		input: &'ctx mut Input,
		flow: &'ctx mut GameFlow,
		win: &'ctx mut Window,
	}
}

impl<'ctx> System<'ctx> for Menu {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self {
			store: None,
		})
	}

	fn update(&mut self, mut dep: Self::Dependencies) -> Result<()> {
		let (row, col, editor) = if let GameState::Menu(Screen::Main { row, col, editor }) = &mut dep.flow.state {
			(row, col, editor)
		} else {
			return Ok(());
		};

		if dep.flow.changed {
			dep.flow.changed = false;

			dep.win.clear();

			self.prepare(dep.win, dep.arc)?;
			self.show(dep.win, dep.cfg, &dep.cache.font_c04, *row, *col);

			dep.win.fade_in();
		}

		if dep.input.keys.is_empty() {
			return Ok(());
		}

		let prev_cfg = dep.cfg.clone();
		let prev_state = (*row, *col, *editor);

		for key in &dep.input.keys {
			if let Some(state) = key_press(key, dep.cfg, &mut dep.ctrl.quit_requested, row, col, editor) {
				self.store = None;

				dep.flow.set(state);
				dep.input.keys.clear();

				dep.win.fade_out();
				dep.win.free();

				return Ok(());
			}

			if dep.ctrl.quit_requested {
				dep.win.fade_out();
				dep.win.free();

				return Ok(());
			}
		}

		if (*row, *col, *editor) != prev_state || dep.cfg != &prev_cfg {
			dep.win.clear();
			self.show(dep.win, dep.cfg, &dep.cache.font_c04, *row, *col);
			dep.win.present();
		}

		Ok(())
	}
}

fn key_press(key: &KeyCode, cfg: &mut Config, quit: &mut bool, row: &mut u8, col: &mut u8, editor: &mut bool) -> Option<GameState> {
	match key {
		KeyCode::Up if *row > 0 => {
			*row -= 1;
			*editor = false;
		}
		KeyCode::Down if *row < 4 => {
			*row += 1;
			*editor = false;
		}
		KeyCode::Left if *col > 0 => {
			*col -= 1;
			*editor = false;
		}
		KeyCode::Right if *col < 2 => {
			*col += 1;
			*editor = false;
		}
		KeyCode::Escape => {
			if *editor {
				*editor = false;
			} else {
				*quit = true;
			}
		}
		KeyCode::Return => {
			match (*row, *col) {
				(0, 0) => {
					*editor = !*editor;
				}
				START_ITEM_POS => {
					return Some(GameState::ModelSelect);
				}
				(0, 2) => {
					*editor = !*editor;
				}
				(1, 0) => {
					switch_option!(cfg.p1_trans, Transmission::Manual, Transmission::Automatic);
				}
				(1, 1) => {
					switch_option!(cfg.race, Race::TimeLimit, Race::Competition);
				}
				(1, 2) => {
					switch_option!(cfg.p2_trans, Transmission::Manual, Transmission::Automatic);
				}
				(2, 0) => {
					switch_option!(cfg.p1_accel, Acceleration::Button, Acceleration::Joystick);
				}
				(2, 1) => {
					cfg.course = match cfg.course {
						Course::T1 => Course::T2,
						Course::T2 => Course::T3,
						Course::T3 => Course::Circular,
						Course::Circular => Course::Unknown,
						Course::Unknown => Course::T1,
					};
				}
				(2, 2) => {
					switch_option!(cfg.p2_accel, Acceleration::Button, Acceleration::Joystick);
				}
				(3, 0) => {
					// Controls
				}
				(3, 1) => {
					switch_option!(cfg.players_num, 1, 2);
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
					return Some(GameState::define_menu());
				}
				_ => {
					unreachable!();
				}
			}
		}
		KeyCode::Backspace if *editor => {
			match (*row, *col) {
				(0, 0) => { cfg.p1_name.pop(); }
				(0, 2) => { cfg.p2_name.pop(); }
				(4, 1) => { cfg.code.pop(); }
				_ => { unreachable!(); }
			}
		}
		_ if *editor => {
			if let Some(c) = key.to_char() {
				match (*row, *col) {
					(0, 0) if cfg.p1_name.len() < 12 => cfg.p1_name.push(c),
					(0, 2) if cfg.p2_name.len() < 12 => cfg.p2_name.push(c),
					(4, 1) if cfg.code.len() < 12 => cfg.code.push(c),
					_ => {}
				}
			}
		}
		_ => {}
	}

	None
}
