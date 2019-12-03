#[derive(Clone, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
pub enum Transmission {
	Manual = 0,
	Automatic = 1,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Acceleration {
	Button = 0,
	Joystick = 1,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Race {
	TimeLimit = 0,
	Competition = 1,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Course {
	T1 = 0,
	T2 = 1,
	T3 = 2,
	Circular = 3,
	Unknown = 4,
}
