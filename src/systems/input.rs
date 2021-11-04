use sdl2::{event::Event, keyboard::Keycode};
use eyre::Result;

use crate::{
	ecs::{
		context::ControlFlow,
		system::System,
	},
	game::input::KeyCode,
	systems::Window,
};

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		ctrl: &'ctx mut ControlFlow,
		win: &'ctx mut Window,
	}
}

pub struct Input {
	pub keys: Vec<KeyCode>,
}

impl<'ctx> System<'ctx> for Input {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self { keys: Vec::new() })
	}

	fn update(&mut self, dep: Self::Dependencies) -> Result<()> {
		self.keys.clear();

		let mut events = dep.win.context.event_pump().unwrap();

		for event in events.poll_iter() {
			match event {
				Event::Quit { .. } => dep.ctrl.quit_requested = true,
				Event::KeyDown { keycode: Some(key), .. } => {
					if let Some(key) = convert(key) {
						self.keys.push(key);
					}
				}
				_ => {}
			}
		}

		Ok(())
	}
}

fn convert(key: Keycode) -> Option<KeyCode> {
	match key {
		Keycode::A => Some(KeyCode::A),
		Keycode::B => Some(KeyCode::B),
		Keycode::C => Some(KeyCode::C),
		Keycode::D => Some(KeyCode::D),
		Keycode::E => Some(KeyCode::E),
		Keycode::F => Some(KeyCode::F),
		Keycode::G => Some(KeyCode::G),
		Keycode::H => Some(KeyCode::H),
		Keycode::I => Some(KeyCode::I),
		Keycode::J => Some(KeyCode::J),
		Keycode::K => Some(KeyCode::K),
		Keycode::L => Some(KeyCode::L),
		Keycode::M => Some(KeyCode::M),
		Keycode::N => Some(KeyCode::N),
		Keycode::O => Some(KeyCode::O),
		Keycode::P => Some(KeyCode::P),
		Keycode::Q => Some(KeyCode::Q),
		Keycode::R => Some(KeyCode::R),
		Keycode::S => Some(KeyCode::S),
		Keycode::T => Some(KeyCode::T),
		Keycode::U => Some(KeyCode::U),
		Keycode::V => Some(KeyCode::V),
		Keycode::W => Some(KeyCode::W),
		Keycode::X => Some(KeyCode::X),
		Keycode::Y => Some(KeyCode::Y),
		Keycode::Z => Some(KeyCode::Z),
		Keycode::Num0 => Some(KeyCode::Num0),
		Keycode::Num1 => Some(KeyCode::Num1),
		Keycode::Num2 => Some(KeyCode::Num2),
		Keycode::Num3 => Some(KeyCode::Num3),
		Keycode::Num4 => Some(KeyCode::Num4),
		Keycode::Num5 => Some(KeyCode::Num5),
		Keycode::Num6 => Some(KeyCode::Num6),
		Keycode::Num7 => Some(KeyCode::Num7),
		Keycode::Num8 => Some(KeyCode::Num8),
		Keycode::Num9 => Some(KeyCode::Num9),
		Keycode::Left => Some(KeyCode::Left),
		Keycode::Right => Some(KeyCode::Right),
		Keycode::Up => Some(KeyCode::Up),
		Keycode::Down => Some(KeyCode::Down),
		Keycode::Escape => Some(KeyCode::Escape),
		Keycode::Backspace => Some(KeyCode::Backspace),
		Keycode::Return => Some(KeyCode::Return),
		Keycode::Delete => Some(KeyCode::Delete),
		Keycode::Space => Some(KeyCode::Space),
		_ => None,
	}
}
