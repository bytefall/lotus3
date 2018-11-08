use super::{Size, HEIGHT, WIDTH};

pub struct Sprite {
	data: Vec<u8>,
	pub size: Size,
}

impl Sprite {
	pub fn from(data: Vec<u8>) -> Sprite {
		Sprite {
			data,
			size: Size {
				width: WIDTH as u32,
				height: HEIGHT as u32,
			},
		}
	}

	pub fn with_size(mut self, width: u32, height: u32) -> Sprite {
		self.size = Size { width, height };
		self
	}

	pub fn draw<'a>(&'a self, palette: &'a [u8]) -> impl FnOnce(&mut [u8], usize) + 'a {
		let mut iter = self.data.iter();

		move |buffer: &mut [u8], pitch: usize| {
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
	}
}
