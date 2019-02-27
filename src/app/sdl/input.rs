use sdl2::keyboard::Keycode as From;

use crate::app::input::KeyCode as To;

pub fn convert(key: From) -> Option<To> {
	match key {
		From::A => Some(To::A),
		From::B => Some(To::B),
		From::C => Some(To::C),
		From::D => Some(To::D),
		From::E => Some(To::E),
		From::F => Some(To::F),
		From::G => Some(To::G),
		From::H => Some(To::H),
		From::I => Some(To::I),
		From::J => Some(To::J),
		From::K => Some(To::K),
		From::L => Some(To::L),
		From::M => Some(To::M),
		From::N => Some(To::N),
		From::O => Some(To::O),
		From::P => Some(To::P),
		From::Q => Some(To::Q),
		From::R => Some(To::R),
		From::S => Some(To::S),
		From::T => Some(To::T),
		From::U => Some(To::U),
		From::V => Some(To::V),
		From::W => Some(To::W),
		From::X => Some(To::X),
		From::Y => Some(To::Y),
		From::Z => Some(To::Z),
		From::Num0 => Some(To::Num0),
		From::Num1 => Some(To::Num1),
		From::Num2 => Some(To::Num2),
		From::Num3 => Some(To::Num3),
		From::Num4 => Some(To::Num4),
		From::Num5 => Some(To::Num5),
		From::Num6 => Some(To::Num6),
		From::Num7 => Some(To::Num7),
		From::Num8 => Some(To::Num8),
		From::Num9 => Some(To::Num9),
		From::Left => Some(To::Left),
		From::Right => Some(To::Right),
		From::Up => Some(To::Up),
		From::Down => Some(To::Down),
		From::Escape => Some(To::Escape),
		From::Backspace => Some(To::Backspace),
		From::Return => Some(To::Return),
		From::Delete => Some(To::Delete),
		From::Space => Some(To::Space),
		_ => None,
	}
}
