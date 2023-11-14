pub const SCREEN_WIDTH: u32 = 320;
pub const SCREEN_HEIGHT: u32 = 200;
pub const SCREEN_SIZE: Size = Size::wh(SCREEN_WIDTH, SCREEN_HEIGHT);

#[derive(Copy, Clone)]
pub struct Canvas([u32; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]);

impl Canvas {
	pub const fn new() -> Self {
		Self([0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize])
	}

	pub fn get(&self) -> &[u32] {
		&self.0
	}

	pub fn raw(&mut self) -> &mut [u32] {
		&mut self.0
	}

	pub fn raw_at(&mut self, pos: impl Into<Point>) -> &mut [u32] {
		&mut self.0[pos.into().index()..]
	}
}

#[derive(Clone)]
pub struct Point {
	pub x: u32,
	pub y: u32,
}

impl Point {
	pub const fn xy(x: u32, y: u32) -> Self {
		Self { x, y }
	}

	pub fn index(&self) -> usize {
		(self.y * SCREEN_WIDTH + self.x) as usize
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

mod bitmap;
pub mod font;
mod frame;
mod sprite;
mod sprite_font;

pub use self::bitmap::{decode, Bitmap};
pub use self::frame::{Frame, FRAME_BORDER};
pub use self::sprite::Sprite;
pub use self::sprite_font::SpriteFont;
