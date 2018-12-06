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

mod bitmap;
mod font;
mod screen;
mod sprite;

pub use self::bitmap::Bitmap;
pub use self::font::{Font, CHAR_SET_03};
pub use self::screen::Screen;
pub use self::sprite::Sprite;
