use eyre::Result;

use crate::{
	data::Archive,
	ecs::system::System,
	game::state::{GameFlow, GameState},
	graphics::{Size, Sprite, SCREEN_START},
	systems::{Input, Timer, Window},
};

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		arc: &'ctx Archive,
		input: &'ctx mut Input,
		timer: &'ctx mut Timer,
		flow: &'ctx mut GameFlow,
		win: &'ctx mut Window,
	}
}

pub struct AudioTuner;

impl<'ctx> System<'ctx> for AudioTuner {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self {
		})
	}

	fn update(&mut self, mut dep: Self::Dependencies) -> Result<()> {
		if dep.flow.current() != &GameState::AudioTuner {
			return Ok(());
		}

		if !dep.input.keys.is_empty() {
			dep.input.keys.clear();
			dep.win.fade_out();
			dep.win.free();
			dep.flow.set(GameState::main_menu());

			return Ok(());
		}

		if dep.flow.changed {
			dep.flow.changed = false;

			let (bgr, pal) = dep.arc.get_with_palette("I1C")?;
			dep.win.palette = pal;
			dep.win.draw(&Sprite::from(bgr)).show(SCREEN_START);

			let mut yy = 174;

			for _ in 0..7 {
				dep.win
					.paint(Size::wh(5, 1), |_, c| { c.color(162, 0, 0, 255); c.line((0, 0).into(), (5, 0).into()); })
					.show((168, yy).into());
				yy += 2;
			}

			//dep.win.print(&dep.cache.font_c06, "01:02:03").show((60, 140).into());

			dep.win.fade_in();
		}

		Ok(())
	}
}
