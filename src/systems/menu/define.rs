use generational_arena::Index;
use eyre::Result;

use super::{
	build_frame,
	DEFINE_ITEM_POS,
	FRAME_BORDER,
	FRAME_OFFSET,
	FRAME_SIZE_4R,
	FRAME_SIZE_ST,
};
use crate::{
	data::Archive,
	ecs::{
		context::ControlFlow,
		system::System,
	},
	game::{
		input::KeyCode,
		options::Config,
		state::{GameFlow, GameState, Screen},
	},
	graphics::{font::Font, Point, Sprite, SCREEN_START},
	systems::{Cache, Input, Window},
};

pub struct Menu {
	store: Option<(
		Index,
		Index,
		Index,
	)>,
}

impl Menu {
	fn prepare(&mut self, win: &mut Window, arc: &Archive) {
		self.store = Some((
			win.draw(&Sprite::from(arc.get("I16").unwrap())).id,
			win.paint(FRAME_SIZE_ST, |_, c| build_frame(FRAME_SIZE_ST, c)).id,
			win.paint(FRAME_SIZE_4R, |_, c| build_frame(FRAME_SIZE_4R, c)).id,
		));
	}

	fn show(&self, win: &mut Window, _cfg: &Config, font_c04: &Font, row: u8) {
		let (bg, frame_st, frame_4r) = self.store.unwrap();

		win.show(bg, SCREEN_START);

		win.print(font_c04, "XKXCJGFJH-33").show(Point::xy(117, 56));
		win.print(font_c04, "         -00").show(Point::xy(117, 56 + 15 * 1));
		win.print(font_c04, "         -00").show(Point::xy(117, 56 + 15 * 2));
		win.print(font_c04, "         -00").show(Point::xy(117, 56 + 15 * 3));
		win.print(font_c04, "         -00").show(Point::xy(117, 56 + 15 * 4));
		win.print(font_c04, "         -00").show(Point::xy(117, 56 + 15 * 5));
		win.print(font_c04, "         -00").show(Point::xy(117, 56 + 15 * 6));
		win.print(font_c04, "         -00").show(Point::xy(117, 56 + 15 * 7));
		win.print(font_c04, "         -00").show(Point::xy(117, 56 + 15 * 8));

		// frame should be the last (i.e. on top of everything)
		const COL: u8 = 1;

		let frame = if row == 0 { frame_st } else { frame_4r };
		let width = win.txt_size(frame).unwrap().width;

		win.show(
			frame,
			Point::xy(
				(FRAME_OFFSET.0 + COL as u32 * (width - FRAME_BORDER)) as i32,
				(FRAME_OFFSET.1 + row as u32 * (FRAME_SIZE_ST.height - FRAME_BORDER)) as i32,
			),
		);
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
		Ok(Self { store: None })
	}

	fn update(&mut self, mut dep: Self::Dependencies) -> Result<()> {
		let (row, editor) = if let GameState::Menu(Screen::Define { row, editor }) = &mut dep.flow.state {
			(row, editor)
		} else {
			return Ok(());
		};

		if dep.flow.changed {
			dep.flow.changed = false;

			dep.win.clear();

			self.prepare(dep.win, dep.arc);
			self.show(dep.win, dep.cfg, &dep.cache.font_c04, *row);

			dep.win.fade_in();
		}

		if dep.input.keys.is_empty() {
			return Ok(());
		}

		let prev_cfg = dep.cfg.clone();
		let prev_state = (*row, *editor);

		for key in &dep.input.keys {
			if key_press(key, dep.cfg, row, editor) {
				self.store = None;

				dep.flow.set(GameState::main_menu_pos(DEFINE_ITEM_POS));
				dep.input.keys.clear();

				dep.win.fade_out();
				dep.win.free();

				return Ok(());
			}
		}

		if (*row, *editor) != prev_state || dep.cfg != &prev_cfg {
			dep.win.clear();
			self.show(dep.win, dep.cfg, &dep.cache.font_c04, *row);
			dep.win.present();
		}

		Ok(())
	}

	fn debug_name() -> &'static str {
		file!()
	}
}

fn key_press(key: &KeyCode, _cfg: &mut Config, row: &mut u8, editor: &mut bool) -> bool {
	match (key, *row, *editor) {
		(KeyCode::Down, 0, false) => {
			*row += 1;
		}
		(KeyCode::Up, 1, false) => {
			*row -= 1;
		}
		(KeyCode::Escape, _, true) => {
			*editor = false;
		}
		(KeyCode::Escape, _, false) => {
			return true;
		}
		(KeyCode::Return, 0, false) => {
			return true;
		}
		(KeyCode::Return, 1, _) => {
			*editor = if *editor { false } else { true };
		}
		_ => {}
	}

	false
}
