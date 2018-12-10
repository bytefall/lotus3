use super::{Rgb, Size};

const CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ.()";

const WIDTH: usize = 16;
const HEIGHT: usize = 18;

const HORIZONTAL_SPACE: usize = 14;
const VERTICAL_SPACE: usize = 24;

const PALETTE: &[Rgb] = &[
	Rgb::from(0, 0, 0),
	Rgb::from(48, 48, 48),
	Rgb::from(32, 32, 32),
	Rgb::from(0, 0, 0),
];

pub struct SpriteFont {
	data: Vec<u8>,
}

impl SpriteFont {
	pub fn from(data: Vec<u8>) -> Self {
		Self { data }
	}

	pub fn size(&self, text: &str) -> Size {
		Size {
			width: (text.len() * HORIZONTAL_SPACE) as u32,
			height: ((text.chars().filter(|c| c == &'\n').count() + 1) * VERTICAL_SPACE) as u32,
		}
	}

	pub fn print<'a>(&'a self, text: &'a str) -> impl FnOnce(&mut [u8], usize) + 'a {
		move |buffer: &mut [u8], pitch: usize| {
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
					let mut data = self.data.chunks(WIDTH * HEIGHT).nth(i).unwrap().iter();

					for y in yy..yy + HEIGHT {
						for x in xx..xx + WIDTH {
							let val = *data.next().unwrap() as usize;

							if val == 0 {
								continue;
							}

							let offset = y * pitch + x * 4;

							buffer[offset + 0] = 255;
							buffer[offset + 1] = PALETTE[val].b << 2;
							buffer[offset + 2] = PALETTE[val].g << 2;
							buffer[offset + 3] = PALETTE[val].r << 2;
						}
					}

					xx += HORIZONTAL_SPACE;
				}
			}
		}
	}
}
