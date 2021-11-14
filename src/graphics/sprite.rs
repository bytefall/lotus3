use super::{Drawable, Size, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Sprite {
	data: Vec<u8>,
	size: Size,
}

impl Sprite {
	pub fn from(data: Vec<u8>) -> Self {
		Self {
			data,
			size: Size::wh(SCREEN_WIDTH, SCREEN_HEIGHT),
		}
	}

	pub fn with_size(mut self, size: Size) -> Self {
		self.size = size;
		self
	}
}

impl Drawable for Sprite {
	fn draw(&self, buffer: &mut [u8], pitch: usize, palette: &[u8]) {
		let mut iter = self.data.iter();

		for y in 0..self.size.height {
			for x in 0..self.size.width {
				let offset = y as usize * pitch + x as usize * 3;
				let data = *iter.next().unwrap() as usize;

				buffer[offset + 0] = palette[data * 3 + 0] << 2;
				buffer[offset + 1] = palette[data * 3 + 1] << 2;
				buffer[offset + 2] = palette[data * 3 + 2] << 2;
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
