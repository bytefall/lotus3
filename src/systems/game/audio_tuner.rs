use eyre::Result;
use winit::event::VirtualKeyCode;

use crate::{
	data::Archive,
	ecs::{context::ControlFlow, system::System},
	game::state::{GameFlow, GameState},
	graphics::{Sprite, SCREEN_START},
	systems::{Timer, Window},
};

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		arc: &'ctx Archive,
		timer: &'ctx mut Timer,
		ctrl: &'ctx mut ControlFlow,
		flow: &'ctx mut GameFlow,
		win: &'ctx mut Window,
	}
}

struct Assets {
	bgr: Sprite,
}

#[derive(Default)]
pub struct AudioTuner {
	assets: Option<Assets>,
}

impl<'ctx> System<'ctx> for AudioTuner {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self::default())
	}

	fn update(&mut self, dep: Self::Dependencies) -> Result<()> {
		if dep.flow.current() != &GameState::AudioTuner {
			return Ok(());
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Escape) {
			dep.flow.set(GameState::main_menu());
			dep.win.fade_out();
			dep.ctrl.input = winit_input_helper::WinitInputHelper::new();

			return Ok(());
		}

		if dep.flow.changed {
			dep.flow.changed = false;

			let (pal, a) = load(dep.arc)?;
			dep.win.palette = pal;
			self.assets = Some(a);

			render(dep.win, self.assets.as_ref().unwrap());

			dep.win.fade_in();
		}

		Ok(())
	}
}

fn load(arc: &Archive) -> Result<(Vec<u8>, Assets)> {
	let (bgr, pal) = arc.get_with_palette("I1C")?;

	Ok((pal, Assets { bgr: Sprite::from(bgr) }))
}

fn render(win: &mut Window, a: &Assets) {
	win.draw(&a.bgr, SCREEN_START);
}
