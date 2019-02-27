pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 200;
pub const SCREEN_START: Point = Point { x: 0, y: 0 };

pub struct Point {
	pub x: i32,
	pub y: i32,
}

#[allow(unused_macros)]
macro_rules! point {
	($x:expr, $y:expr) => {
		Point { x: $x, y: $y }
	}
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

pub trait Drawable {
	fn draw(&self, buffer: &mut [u8], pitch: usize, palette: &[u8]);

	fn width(&self) -> u32;

	fn height(&self) -> u32;
}

pub trait Printable {
	fn print(&self, buffer: &mut [u8], pitch: usize, palette: &[u8], text: &str);

	fn width(&self, text: &str) -> u32;

	fn height(&self, text: &str) -> u32;
}

mod bitmap;
pub mod font;
mod sprite;
mod sprite_font;

pub use self::bitmap::{decode, Bitmap};
pub use self::sprite::Sprite;
pub use self::sprite_font::SpriteFont;
