use super::{Drawable, Point, Size, SCREEN_SIZE};

pub struct Sprite {
	data: Vec<u8>,
	size: Size,
}

impl Sprite {
	pub fn from(data: Vec<u8>) -> Self {
		Self {
			data,
			size: SCREEN_SIZE,
		}
	}

	pub fn with_size(mut self, size: Size) -> Self {
		self.size = size;
		self
	}
}

impl Drawable for Sprite {
	fn draw(&self, buffer: &mut [u8], palette: &[u8]) {
		let mut iter = self.data.iter();

		for y in 0..self.size.height {
			for x in 0..self.size.width {
				let data = *iter.next().unwrap() as usize;
				let buffer = &mut buffer[Point::xy(x, y).range()];

				buffer[0] = palette[data * 3 + 0] << 2;
				buffer[1] = palette[data * 3 + 1] << 2;
				buffer[2] = palette[data * 3 + 2] << 2;
				buffer[3] = 255;
			}
		}
	}

	fn width(&self) -> u32 {
		self.size.width
	}

	fn height(&self) -> u32 {
		self.size.height
	}
}
