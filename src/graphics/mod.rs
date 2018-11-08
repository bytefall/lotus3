pub struct Point {
	pub x: i32,
	pub y: i32,
}

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 200;

pub struct Size {
	pub width: u32,
	pub height: u32,
}

mod screen;
mod sprite;

pub use self::screen::Screen;
pub use self::sprite::Sprite;
