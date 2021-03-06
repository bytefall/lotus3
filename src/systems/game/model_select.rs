use crate::{
	data::Archive,
	ecs::{
		errors::{Error, Result},
		system::System,
	},
	game::{
		input::KeyCode,
		options::Model,
		state::{GameFlow, GameState},
	},
	graphics::{Point, Size, Sprite, SCREEN_START},
	systems::{Input, Timer, Window},
};
use generational_arena::Index;
use std::time::Duration;

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		arc: &'ctx Archive,
		input: &'ctx mut Input,
		timer: &'ctx mut Timer,
		flow: &'ctx mut GameFlow,
		win: &'ctx mut Window,
	}
}

pub struct ModelSelect {
	model: Model,
	frame: usize,
	ids: Vec<Index>,
}

impl<'ctx> System<'ctx> for ModelSelect {
	type Dependencies = Dependencies<'ctx>;
	type Error = Error;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self {
			model: Model::Esprit,
			frame: 0,
			ids: Vec::new(),
		})
	}

	fn update(&mut self, mut dep: Self::Dependencies) -> Result<()> {
		if dep.flow.current() != &GameState::ModelSelect {
			return Ok(());
		}

		if let Some(key) = dep.input.keys.first() {
			match key {
				KeyCode::Left => {
					self.model = self.model.prev();
					self.ids.clear();

					dep.win.fade_out();
					dep.win.free();
				}
				KeyCode::Right => {
					self.model = self.model.next();
					self.ids.clear();

					dep.win.fade_out();
					dep.win.free();
				}
				KeyCode::Return | KeyCode::Escape => {
					if matches!(key, KeyCode::Return) {
						dep.flow.set(GameState::main_menu());
					} else {
						dep.flow.set(GameState::main_menu());
					};

					dep.win.fade_out();
					dep.win.free();

					self.model = Model::Esprit;
					self.frame = 0;
					self.ids.clear();

					return Ok(());
				}
				_ => {}
			}
		}

		dep.input.keys.clear();

		if dep.flow.changed {
			dep.flow.changed = false;
		}

		if self.ids.is_empty() {
			self.frame = 0;

			let (bgr_key, ani_key) = KEYS[self.model as usize];

			let (bgr, pal) = dep.arc.get_with_palette(bgr_key).unwrap();
			dep.win.palette = pal;
			dep.win.draw(&Sprite::from(bgr)).show(SCREEN_START);
			dep.win.fade_in();

			let ani = dep.arc.get(ani_key).unwrap();

			for dat in ani.chunks((ANIM_SIZE.width * ANIM_SIZE.height) as usize) {
				self.ids.push(dep.win.draw(&Sprite::from(dat.to_vec()).with_size(ANIM_SIZE)).id);
			}
		} else {
			dep.win.hide(self.ids[self.frame]);
			self.frame = if self.frame + 1 < self.ids.len() { self.frame + 1 } else { 0 };
			dep.win.show(self.ids[self.frame], ANIM_POS);

			dep.win.present();
		}

		dep.timer.sleep(ANIM_DELAY);

		Ok(())
	}

	fn debug_name() -> &'static str {
		file!()
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
