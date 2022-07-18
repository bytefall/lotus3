pub const SCREEN_WIDTH: u32 = 320;
pub const SCREEN_HEIGHT: u32 = 200;
pub const SCREEN_BPP: u32 = 4;
pub const SCREEN_START: Point = Point::xy(0, 0);
pub const SCREEN_SIZE: Size = Size::wh(SCREEN_WIDTH, SCREEN_HEIGHT);

#[derive(Clone)]
pub struct Point {
	pub x: u32,
	pub y: u32,
}

impl Point {
	pub const fn xy(x: u32, y: u32) -> Self {
		Self { x, y }
	}

	pub fn range(&self) -> std::ops::RangeFrom<usize> {
		((self.y * SCREEN_WIDTH + self.x) * SCREEN_BPP) as usize..
	}
}

impl From<(u32, u32)> for Point {
	fn from((x, y): (u32, u32)) -> Self {
		Self::xy(x, y)
	}
}

#[derive(Debug)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl Color {
	pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
		Self { r, g, b }
	}
}

impl From<(u8, u8, u8)> for Color {
	fn from((r, g, b): (u8, u8, u8)) -> Self {
		Self::rgb(r, g, b)
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
	fn draw(&self, buffer: &mut [u8], palette: &[u8]);

	fn width(&self) -> u32;

	fn height(&self) -> u32;
}

pub trait Printable {
	fn print(&self, buffer: &mut [u8], palette: &[u8], text: &str);

	fn width(&self, text: &str) -> u32;

	fn height(&self, text: &str) -> u32;
}

pub struct Canvas(Vec<u8>);

impl Canvas {
	pub fn draw(&mut self, sprite: &dyn Drawable, pal: &[u8], pos: Point) {
		sprite.draw(&mut self.0[pos.range()], pal);
	}

	pub fn print(&mut self, text: &str, font: &dyn Printable, pal: &[u8], pos: Point) {
		font.print(&mut self.0[pos.range()], pal, text);
	}

	pub fn point(&mut self, color: Color, pos: Point) {
		let buffer = &mut self.0[pos.range()];

		buffer[0] = color.r << 2;
		buffer[1] = color.g << 2;
		buffer[2] = color.b << 2;
		buffer[3] = 255;
	}

	pub fn get(&self) -> &[u8] {
		&self.0
	}
}

impl Default for Canvas {
	fn default() -> Self {
		Self(vec![0u8; (SCREEN_WIDTH * SCREEN_HEIGHT * SCREEN_BPP) as usize])
	}
}

pub enum FadeType {
	In,
	Out,
}

mod bitmap;
pub mod font;
mod frame;
mod sprite;
mod sprite_font;

pub use self::bitmap::{decode, Bitmap};
pub use self::frame::{Frame, FRAME_BORDER};
pub use self::sprite::Sprite;
pub use self::sprite_font::SpriteFont;
