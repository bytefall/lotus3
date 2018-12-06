use super::{Bitmap, Point, Size};

pub struct Font {
	char_set: CharSet,
	bitmap: Bitmap,
}

impl Font {
	pub fn from(char_set: CharSet, data: Vec<u8>) -> Self {
		Self {
			char_set,
			bitmap: Bitmap::from(data, 0, 0),
		}
	}

	pub fn size(&self, text: &str) -> Size {
		Size {
			width: (text.len() * self.char_set.h_space) as u32,
			height: ((text.chars().filter(|c| c == &'\n').count() + 1) * self.char_set.v_space) as u32,
		}
	}

	pub fn print<'a>(&'a self, palette: &'a [u8], text: &'a str) -> impl FnOnce(&mut [u8], usize) + 'a {
		move |mut buffer: &mut [u8], pitch: usize| {
			let mut xx = 0;
			let mut yy = 0;

			for c in text.chars() {
				if c == ' ' {
					xx += self.char_set.h_space;
					continue;
				}

				if c == '\n' {
					xx = 0;
					yy += self.char_set.v_space;
					continue;
				}

				if let Some(i) = self.char_set.chars.find(c) {
					self.bitmap.draw(i, Point { x: xx as i32, y: yy as i32 }, &mut buffer, pitch, &palette);

					xx += self.char_set.h_space;
				}
			}
		}
	}
}

pub struct CharSet {
	chars: &'static str,
	h_space: usize,
	v_space: usize,
}
/*
pub const CHAR_SET_00: CharSet = CharSet {
	chars: "0123456789\u{1}\u{2}\u{3}\u{4}\u{5}",
	h_space: 16,
	v_space: 18,
};

pub const CHAR_SET_01: CharSet = CharSet {
	chars: "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ—-",
	h_space: 9,
	v_space: 10,
};
*/
pub const CHAR_SET_03: CharSet = CharSet {
	chars: "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-‾",
	h_space: 9,
	v_space: 10,
};
/*
pub const CHAR_SET_04: CharSet = CharSet {
	chars: "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-‾",
	h_space: 9,
	v_space: 10,
};
*/
