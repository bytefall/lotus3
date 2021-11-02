#[derive(Clone)]
pub enum KeyCode {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	Num0,
	Num1,
	Num2,
	Num3,
	Num4,
	Num5,
	Num6,
	Num7,
	Num8,
	Num9,
	Left,
	Right,
	Up,
	Down,
	Escape,
	Backspace,
	Return,
	Delete,
	Space,
}

impl KeyCode {
	pub fn to_char(&self) -> Option<char> {
		match self {
			KeyCode::A => Some('A'),
			KeyCode::B => Some('B'),
			KeyCode::C => Some('C'),
			KeyCode::D => Some('D'),
			KeyCode::E => Some('E'),
			KeyCode::F => Some('F'),
			KeyCode::G => Some('G'),
			KeyCode::H => Some('H'),
			KeyCode::I => Some('I'),
			KeyCode::J => Some('J'),
			KeyCode::K => Some('K'),
			KeyCode::L => Some('L'),
			KeyCode::M => Some('M'),
			KeyCode::N => Some('N'),
			KeyCode::O => Some('O'),
			KeyCode::P => Some('P'),
			KeyCode::Q => Some('Q'),
			KeyCode::R => Some('R'),
			KeyCode::S => Some('S'),
			KeyCode::T => Some('T'),
			KeyCode::U => Some('U'),
			KeyCode::V => Some('V'),
			KeyCode::W => Some('W'),
			KeyCode::X => Some('X'),
			KeyCode::Y => Some('Y'),
			KeyCode::Z => Some('Z'),
			KeyCode::Num0 => Some('0'),
			KeyCode::Num1 => Some('1'),
			KeyCode::Num2 => Some('2'),
			KeyCode::Num3 => Some('3'),
			KeyCode::Num4 => Some('4'),
			KeyCode::Num5 => Some('5'),
			KeyCode::Num6 => Some('6'),
			KeyCode::Num7 => Some('7'),
			KeyCode::Num8 => Some('8'),
			KeyCode::Num9 => Some('9'),
			_ => None,
		}
	}
}
