pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 200;

pub struct Point {
	pub x: i32,
	pub y: i32,
}

pub struct Rgb {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl Rgb {
	pub const fn from(r: u8, g: u8, b: u8) -> Self {
		Rgb { r, g, b }
	}
}

pub struct Size {
	pub width: u32,
	pub height: u32,
}

mod bitmap;
mod font;
mod screen;
mod sprite;
mod sprite_font;

pub use self::bitmap::{decode, Bitmap};
pub use self::font::{Font, CHAR_SET_03};
pub use self::screen::Screen;
pub use self::sprite::Sprite;
pub use self::sprite_font::SpriteFont;
