pub const SCREEN_WIDTH: u32 = 320;
pub const SCREEN_HEIGHT: u32 = 200;
pub const SCREEN_BPP: usize = 4;
pub const SCREEN_START: Point = Point::xy(0, 0);

#[derive(Clone)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}

impl Point {
	pub const fn xy(x: i32, y: i32) -> Self {
		Self { x, y }
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

impl Size {
	pub const fn wh(width: u32, height: u32) -> Self {
		Self { width, height }
	}
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

pub trait PaintCanvas {
	fn color(&mut self, r: u8, g: u8, b: u8, a: u8);

	fn point(&mut self, point: Point);

	fn line(&mut self, start: Point, end: Point);
}

pub type PaintFn = dyn FnOnce(&[u8], &mut dyn PaintCanvas);

mod bitmap;
pub mod font;
mod sprite;
mod sprite_font;

pub use self::bitmap::{decode, Bitmap};
pub use self::sprite::Sprite;
pub use self::sprite_font::SpriteFont;
