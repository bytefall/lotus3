use generational_arena::Index;
use eyre::Result;

use crate::{
	data::Archive,
	ecs::system::System,
	game::{
		input::KeyCode,
		state::{GameFlow, GameState},
	},
	graphics::{Size, Sprite, SCREEN_START},
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
	code: String,
}

impl<'ctx> System<'ctx> for Protection {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self{
			input_id: None,
			code: String::new(),
		})
	}

	fn update(&mut self, mut dep: Self::Dependencies) -> Result<()> {
		if dep.flow.state != GameState::Protection {
			return Ok(());
		}

		let mut key_pressed = false;

		for key in &dep.input.keys {
			match key.to_char() {
				Some(c) => {
					if self.code.len() < 3 {
						self.code.push(c);
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
					KeyCode::Backspace if self.code.pop().is_some() => {
						key_pressed = true;
					}
					_ => {}
				},
			}
		}

		if key_pressed {
			let prev = self
				.input_id
				.replace(dep.win.print(&dep.cache.font_c03, &self.code).show((150, 165).into()).id);

			if let Some(id) = prev {
				dep.win.remove(id);
			}

			dep.win.present();
		}

		if dep.flow.changed {
			dep.flow.changed = false;

			show_protection_screen(&mut dep)?;
		}

		Ok(())
	}
}

const HELMET_SIZE: Size = Size::wh(48, 40); // 24 helmets of 48x40 bytes each

fn show_protection_screen(dep: &mut Dependencies) -> Result<()> {
	let i22 = dep.arc.get("I22")?;
	let mut i22 = i22.chunks((HELMET_SIZE.width * HELMET_SIZE.height) as usize);

	let helmets = (
		Sprite::from(i22.next().unwrap().to_vec()).with_size(HELMET_SIZE),
		Sprite::from(i22.next().unwrap().to_vec()).with_size(HELMET_SIZE),
	);

	let (i21, pal) = dep.arc.get_with_palette("I21")?;

	dep.win.clear();
	dep.win.palette = pal;
	dep.win.draw(&Sprite::from(i21)).show(SCREEN_START);
	dep.win.draw(&helmets.0).show((141, 13).into());
	dep.win.draw(&helmets.1).show((141, 73).into());
	dep.win.print(&dep.cache.font_c03, "ENTER CODE FOR WINDOW 47").show((60, 140).into());
	dep.win.present();

	Ok(())
}
