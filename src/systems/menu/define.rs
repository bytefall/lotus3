use eyre::Result;
use winit::event::VirtualKeyCode;

use super::{
	DEFINE_ITEM_POS,
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
		options::Config,
		state::{GameFlow, GameState, Screen},
	},
	graphics::{Drawable as _, font::Font, Sprite, SCREEN_START, Frame, FRAME_BORDER},
	systems::{Cache, Window},
};

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
		let (row, editor) = if let GameState::Menu(Screen::Define { row, editor }) = &mut dep.flow.state {
			(row, editor)
		} else {
			return Ok(());
		};

		if dep.flow.changed {
			dep.flow.changed = false;

			dep.win.clear();

			self.assets = Some(load(dep.arc)?);
			render(dep.win, self.assets.as_ref().unwrap(), dep.cfg, &dep.cache.font_c04, *row);

			dep.win.fade_in();
		}

		let prev_cfg = dep.cfg.clone();
		let prev_state = (*row, *editor);
		let mut quit = false;

		if !*editor {
			match *row {
				0 if dep.ctrl.input.key_pressed(VirtualKeyCode::Down) => *row += 1,
				1 if dep.ctrl.input.key_pressed(VirtualKeyCode::Up) => *row -= 1,
				_ => {},
			}
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Escape) {
			if *editor {
				*editor = false;
			} else {
				quit = true;
			}
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Return) {
			match row {
				0 => quit = true,
				1 => *editor = !*editor,
				_ => {}
			}
		}

		if quit {
			self.assets = None;

			dep.flow.set(GameState::main_menu_pos(DEFINE_ITEM_POS));
			dep.win.fade_out();
			dep.ctrl.input = winit_input_helper::WinitInputHelper::new();

			return Ok(());
		}

		if (*row, *editor) != prev_state || dep.cfg != &prev_cfg {
			dep.win.clear();
			render(dep.win, self.assets.as_ref().unwrap(), dep.cfg, &dep.cache.font_c04, *row);
			dep.win.present();
		}

		Ok(())
	}
}

fn load(arc: &Archive) -> Result<Assets> {
	Ok(Assets {
		bgr: Sprite::from(arc.get("I16")?),
	})
}

fn render(win: &mut Window, a: &Assets, _cfg: &Config, font: &Font, row: u8) {
	win.draw(&a.bgr, SCREEN_START);

	win.print("XKXCJGFJH-33", font, (117, 56).into());
	win.print("         -00", font, (117, 56 + 15 * 1).into());
	win.print("         -00", font, (117, 56 + 15 * 2).into());
	win.print("         -00", font, (117, 56 + 15 * 3).into());
	win.print("         -00", font, (117, 56 + 15 * 4).into());
	win.print("         -00", font, (117, 56 + 15 * 5).into());
	win.print("         -00", font, (117, 56 + 15 * 6).into());
	win.print("         -00", font, (117, 56 + 15 * 7).into());
	win.print("         -00", font, (117, 56 + 15 * 8).into());

	const COL: u8 = 1;

	let frame = Frame::new(if row == 0 { FRAME_SIZE_ST } else { FRAME_SIZE_4R });
	let pos = (
		(COL as u32 * (frame.width() - FRAME_BORDER + 1) + FRAME_OFFSET.0),
		(row as u32 * (FRAME_SIZE_ST.height - FRAME_BORDER + 1) + FRAME_OFFSET.1),
	).into();

	win.draw(&frame, pos);
}
