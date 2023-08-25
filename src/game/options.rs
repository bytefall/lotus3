#[derive(Clone, PartialEq, Eq)]
pub struct Config {
	pub p1_name: String,
	pub p1_trans: Transmission,
	pub p1_accel: Acceleration,
	pub p2_name: String,
	pub p2_trans: Transmission,
	pub p2_accel: Acceleration,
	pub race: Race,
	pub course: Course,
	pub players_num: u8,
	pub code: String,
}

impl Config {
	pub fn new() -> Self {
		Self {
			p1_name: "PLAYER 1".to_string(),
			p1_trans: Transmission::Automatic,
			p1_accel: Acceleration::Button,
			p2_name: "PLAYER 2".to_string(),
			p2_trans: Transmission::Automatic,
			p2_accel: Acceleration::Button,
			race: Race::TimeLimit,
			course: Course::T1,
			players_num: 1,
			code: "VBJD D   -99".to_string(),
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Transmission {
	Manual = 0,
	Automatic = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Acceleration {
	Button = 0,
	Joystick = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Race {
	TimeLimit = 0,
	Competition = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Course {
	T1 = 0,
	T2 = 1,
	T3 = 2,
	Circular = 3,
	Unknown = 4,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Model {
	Esprit = 0,
	Elan = 1,
	M200 = 2,
}

impl Default for Model {
	fn default() -> Self {
		Self::Esprit
	}
}

impl Model {
	pub fn next(&self) -> Self {
		match self {
			Self::Esprit => Self::Elan,
			Self::Elan => Self::M200,
			Self::M200 => Self::Esprit,
		}
	}

	pub fn prev(&self) -> Self {
		match self {
			Self::Esprit => Self::M200,
			Self::Elan => Self::Esprit,
			Self::M200 => Self::Elan,
		}
	}
}
