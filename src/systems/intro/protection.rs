use eyre::Result;
use winit::event::VirtualKeyCode;
use winit_input_helper::TextChar;

use crate::{
	data::Archive,
	ecs::{context::ControlFlow, system::System},
	game::state::{GameFlow, GameState},
	graphics::{font::Font, Size, Sprite, SCREEN_START},
	systems::{Cache, Window},
};

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		arc: &'ctx Archive,
		cache: &'ctx Cache,
		ctrl: &'ctx mut ControlFlow,
		flow: &'ctx mut GameFlow,
		win: &'ctx mut Window,
	}
}

struct Assets {
	bgr: Sprite,
	helmet1: Sprite,
	helmet2: Sprite,
}

#[derive(Default)]
pub struct Protection {
	assets: Option<Assets>,
	code: String,
}

const HELMET_SIZE: Size = Size::wh(48, 40); // 24 helmets of 48x40 bytes each

impl<'ctx> System<'ctx> for Protection {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self::default())
	}

	fn update(&mut self, dep: Self::Dependencies) -> Result<()> {
		if dep.flow.current() != &GameState::Protection {
			return Ok(());
		}

		let mut first_time = false;

		if dep.flow.changed {
			dep.flow.changed = false;
			first_time = true;

			let (pal, a) = load(dep.arc)?;
			dep.win.palette = pal;
			self.assets = Some(a);
		}

		let mut key_pressed = false;

		for key in dep.ctrl.input.text() {
			match key {
				TextChar::Char(c) if self.code.len() < 3 => {
					if c.is_ascii_alphabetic() || c.is_ascii_digit() {
						self.code.push(c.to_ascii_uppercase());

						key_pressed = true;
					}
				}
				TextChar::Back if self.code.pop().is_some() => {
					key_pressed = true;
				}
				_ => {}
			}
		}

		if dep.ctrl.input.key_pressed(VirtualKeyCode::Return) || dep.ctrl.input.key_pressed(VirtualKeyCode::Escape) {
			dep.flow.set(GameState::Intro);
			dep.win.fade_out();
			dep.ctrl.input = winit_input_helper::WinitInputHelper::new();

			return Ok(());
		}

		if key_pressed || first_time {
			render(dep.win, self.assets.as_ref().unwrap(), &self.code, &dep.cache.font_c03);
		}

		Ok(())
	}
}

fn load(arc: &Archive) -> Result<(Vec<u8>, Assets)> {
	let i22 = arc.get("I22")?;
	let mut i22 = i22.chunks((HELMET_SIZE.width * HELMET_SIZE.height) as usize);

	let (i21, pal) = arc.get_with_palette("I21")?;

	let a = Assets {
		helmet1: Sprite::from(i22.next().unwrap().to_vec()).with_size(HELMET_SIZE),
		helmet2: Sprite::from(i22.next().unwrap().to_vec()).with_size(HELMET_SIZE),
		bgr: Sprite::from(i21),
	};

	Ok((pal, a))
}

fn render(win: &mut Window, a: &Assets, code: &str, font: &Font) {
	win.draw(&a.bgr, SCREEN_START);
	win.draw(&a.helmet1, (141, 13).into());
	win.draw(&a.helmet2, (141, 73).into());
	win.print("ENTER CODE FOR WINDOW 47", font, (60, 140).into());
	win.print(code, font, (150, 165).into());
	win.present();
}
