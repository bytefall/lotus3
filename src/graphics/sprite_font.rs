use super::{Color, Point, Printable};

const CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ.()";

const WIDTH: u32 = 16;
const HEIGHT: u32 = 18;

const HORIZONTAL_SPACE: u32 = 14;
const VERTICAL_SPACE: u32 = 24;

const PALETTE: &[Color] = &[
	Color::rgb(0, 0, 0),
	Color::rgb(48, 48, 48),
	Color::rgb(32, 32, 32),
	Color::rgb(0, 0, 0),
];

pub struct SpriteFont {
	data: Vec<u8>,
}

impl SpriteFont {
	pub fn from(data: Vec<u8>) -> Self {
		Self { data }
	}
}

impl Printable for SpriteFont {
	fn print(&self, buffer: &mut [u8], _palette: &[u8], text: &str) {
		let mut xx = 0;
		let mut yy = 0;

		for c in text.chars() {
			if c == ' ' {
				xx += HORIZONTAL_SPACE;
				continue;
			}

			if c == '\n' {
				xx = 0;
				yy += VERTICAL_SPACE;
				continue;
			}

			if let Some(i) = CHARS.find(c) {
				let mut data = self.data.chunks((WIDTH * HEIGHT) as usize).nth(i).unwrap().iter();

				for y in yy..yy + HEIGHT {
					for x in xx..xx + WIDTH {
						let val = *data.next().unwrap() as usize;

						if val == 0 {
							continue;
						}

						let buffer = &mut buffer[Point::xy(x, y).range()];

						buffer[0] = PALETTE[val].r << 2;
						buffer[1] = PALETTE[val].g << 2;
						buffer[2] = PALETTE[val].b << 2;
						buffer[3] = 255;
					}
				}

				xx += HORIZONTAL_SPACE;
			}
		}
	}

	fn width(&self, text: &str) -> u32 {
		text.len() as u32 * HORIZONTAL_SPACE
	}

	fn height(&self, text: &str) -> u32 {
		(text.chars().filter(|c| c == &'\n').count() as u32 + 1) * VERTICAL_SPACE
	}
}
