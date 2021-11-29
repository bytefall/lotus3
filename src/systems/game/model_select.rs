use eyre::Result;
use std::time::Duration;
use winit::event::VirtualKeyCode;

use crate::{
	data::Archive,
	ecs::{context::ControlFlow, system::System},
	game::{
		options::Model,
		state::{GameFlow, GameState},
	},
	graphics::{Point, Size, Sprite, SCREEN_START},
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
	anim: Vec<Sprite>,
}

#[derive(Default)]
pub struct ModelSelect {
	model: Model,
	frame: usize,
	assets: Option<Assets>,
}

impl<'ctx> System<'ctx> for ModelSelect {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self::default())
	}

	fn update(&mut self, dep: Self::Dependencies) -> Result<()> {
		if dep.flow.current() != &GameState::ModelSelect {
			return Ok(());
		}

		if dep.flow.changed {
			dep.flow.changed = false;
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Left) {
			self.model = self.model.prev();
			self.assets = None;

			dep.win.fade_out();
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Right) {
			self.model = self.model.next();
			self.assets = None;

			dep.win.fade_out();
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Return) || dep.ctrl.input.key_pressed(VirtualKeyCode::Escape) {
			if dep.ctrl.input.key_pressed(VirtualKeyCode::Return) {
				dep.flow.set(GameState::AudioTuner);
			} else {
				dep.flow.set(GameState::main_menu());
			};

			dep.ctrl.input = winit_input_helper::WinitInputHelper::new();
			dep.win.fade_out();

			*self = Self::default();

			return Ok(());
		}

		if self.assets.is_none() {
			self.frame = 0;

			let (pal, a) = load(self.model, dep.arc)?;
			dep.win.palette = pal;
			self.assets = Some(a);

			render(dep.win, self.assets.as_ref().unwrap(), self.frame);

			dep.win.fade_in();
		} else {
			self.frame = if self.frame + 1 < self.assets.as_ref().unwrap().anim.len() {
				self.frame + 1
			} else {
				0
			};

			render(dep.win, self.assets.as_ref().unwrap(), self.frame);

			dep.win.present();
		}

		dep.timer.sleep(ANIM_DELAY);

		Ok(())
	}
}

const KEYS: &[(&str, &str)] = &[
	("I1E", "I1D"), // Esprit S4
	("I11", "I10"), // Elan SE
	("I13", "I12"), // M200
];

const ANIM_DELAY: Duration = Duration::from_millis(100);
const ANIM_SIZE: Size = Size::wh(88, 24); // 16 icons of 88x24 bytes each
const ANIM_POS: Point = Point::xy(91, 97);

fn load(model: Model, arc: &Archive) -> Result<(Vec<u8>, Assets)> {
	let (bgr_key, ani_key) = KEYS[model as usize];

	let (bgr, pal) = arc.get_with_palette(bgr_key)?;
	let ani = arc.get(ani_key)?;

	let a = Assets {
		bgr: Sprite::from(bgr),
		anim: ani
			.chunks((ANIM_SIZE.width * ANIM_SIZE.height) as usize)
			.map(|i| Sprite::from(i.to_vec()).with_size(ANIM_SIZE))
			.collect(),
	};

	Ok((pal, a))
}

fn render(win: &mut Window, a: &Assets, frame: usize) {
	win.draw(&a.bgr, SCREEN_START);
	win.draw(&a.anim[frame], ANIM_POS);
}
