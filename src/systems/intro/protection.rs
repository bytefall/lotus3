use generational_arena::Index;
use eyre::Result;

use crate::{
	data::Archive,
	ecs::system::System,
	game::{
		input::KeyCode,
		state::{GameFlow, GameState},
	},
	graphics::{Point, Size, Sprite, SCREEN_START},
	systems::{Cache, Input, Window},
};

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		arc: &'ctx Archive,
		cache: &'ctx Cache,
		input: &'ctx mut Input,
		flow: &'ctx mut GameFlow,
		win: &'ctx mut Window,
	}
}

pub struct Protection {
	input_id: Option<Index>,
}

impl<'ctx> System<'ctx> for Protection {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self { input_id: None })
	}

	fn update(&mut self, mut dep: Self::Dependencies) -> Result<()> {
		let code = if let GameState::Protection(code) = &mut dep.flow.state {
			code
		} else {
			return Ok(());
		};

		let mut key_pressed = false;

		for key in &dep.input.keys {
			match key.to_char() {
				Some(c) => {
					if code.len() < 3 {
						code.push(c);
						key_pressed = true;
					}
				}
				None => match key {
					KeyCode::Return | KeyCode::Escape => {
						dep.input.keys.clear();
						dep.win.fade_out();
						dep.win.free();
						dep.flow.set(GameState::Intro);

						self.input_id = None;

						return Ok(());
					}
					KeyCode::Backspace => {
						code.pop();
						key_pressed = true;
					}
					_ => {}
				},
			}
		}

		if key_pressed {
			let prev = self
				.input_id
				.replace(dep.win.print(&dep.cache.font_c03, &code).show(Point::xy(150, 165)).id);

			if let Some(id) = prev {
				dep.win.remove(id);
			}

			dep.win.present();
		}

		if dep.flow.changed {
			dep.flow.changed = false;

			show_protection_screen(&mut dep);
		}

		Ok(())
	}

	fn debug_name() -> &'static str {
		file!()
	}
}

const HELMET_SIZE: Size = Size::wh(48, 40); // 24 helmets of 48x40 bytes each

fn show_protection_screen(dep: &mut Dependencies) {
	let i22 = dep.arc.get("I22").unwrap();
	let mut i22 = i22.chunks((HELMET_SIZE.width * HELMET_SIZE.height) as usize);

	let helmets = (
		Sprite::from(i22.next().unwrap().to_vec()).with_size(HELMET_SIZE),
		Sprite::from(i22.next().unwrap().to_vec()).with_size(HELMET_SIZE),
	);

	let (i21, pal) = dep.arc.get_with_palette("I21").unwrap();

	dep.win.clear();
	dep.win.palette = pal;
	dep.win.draw(&Sprite::from(i21)).show(SCREEN_START);
	dep.win.draw(&helmets.0).show(Point::xy(141, 13));
	dep.win.draw(&helmets.1).show(Point::xy(141, 73));
	dep.win.print(&dep.cache.font_c03, "ENTER CODE FOR WINDOW 47").show(Point::xy(60, 140));
	dep.win.present();
}
