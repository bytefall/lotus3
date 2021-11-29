use super::{Drawable, Point, Size};

pub const FRAME_BORDER: u32 = 4;

pub struct Frame {
	size: Size,
}

impl Frame {
	pub fn new(size: Size) -> Self {
		Self {
			size,
		}
	}
}

impl Drawable for Frame {
	fn draw(&self, buffer: &mut [u8], pal: &[u8]) {
		let width = self.width() - FRAME_BORDER;		// inner width
		let height = self.height() - FRAME_BORDER;		// inner height

		use FrameColor::*;

		// top left corner

		put_pixel((2, 0), Black, buffer, pal);
		put_pixel((3, 0), Black, buffer, pal);

		put_pixel((1, 1), Black, buffer, pal);
		put_pixel((2, 1), Maroon, buffer, pal);
		put_pixel((3, 1), Red, buffer, pal);

		put_pixel((0, 2), Black, buffer, pal);
		put_pixel((1, 2), Maroon, buffer, pal);
		put_pixel((2, 2), Red, buffer, pal);
		put_pixel((3, 2), Maroon, buffer, pal);

		put_pixel((0, 3), Black, buffer, pal);
		put_pixel((1, 3), Red, buffer, pal);
		put_pixel((2, 3), Maroon, buffer, pal);
		put_pixel((3, 3), Black, buffer, pal);

		// top right corner

		put_pixel((width + 0, 0), Black, buffer, pal);
		put_pixel((width + 1, 0), Black, buffer, pal);

		put_pixel((width + 0, 1), Red, buffer, pal);
		put_pixel((width + 1, 1), Maroon, buffer, pal);
		put_pixel((width + 2, 1), Black, buffer, pal);

		put_pixel((width + 0, 2), Maroon, buffer, pal);
		put_pixel((width + 1, 2), Red, buffer, pal);
		put_pixel((width + 2, 2), Maroon, buffer, pal);
		put_pixel((width + 3, 2), Black, buffer, pal);

		put_pixel((width + 0, 3), Black, buffer, pal);
		put_pixel((width + 1, 3), Maroon, buffer, pal);
		put_pixel((width + 2, 3), Red, buffer, pal);
		put_pixel((width + 3, 3), Black, buffer, pal);

		// right bottom corner

		put_pixel((width + 0, height), Black, buffer, pal);
		put_pixel((width + 1, height), Maroon, buffer, pal);
		put_pixel((width + 2, height), Red, buffer, pal);
		put_pixel((width + 3, height), Black, buffer, pal);

		put_pixel((width + 0, height + 1), Maroon, buffer, pal);
		put_pixel((width + 1, height + 1), Red, buffer, pal);
		put_pixel((width + 2, height + 1), Maroon, buffer, pal);
		put_pixel((width + 3, height + 1), Black, buffer, pal);

		put_pixel((width + 0, height + 2), Red, buffer, pal);
		put_pixel((width + 1, height + 2), Maroon, buffer, pal);
		put_pixel((width + 2, height + 2), Black, buffer, pal);

		put_pixel((width + 0, height + 3), Black, buffer, pal);
		put_pixel((width + 1, height + 3), Black, buffer, pal);

		// left bottom corner

		put_pixel((0, height), Black, buffer, pal);
		put_pixel((1, height), Red, buffer, pal);
		put_pixel((2, height), Maroon, buffer, pal);
		put_pixel((3, height), Black, buffer, pal);

		put_pixel((0, height + 1), Black, buffer, pal);
		put_pixel((1, height + 1), Maroon, buffer, pal);
		put_pixel((2, height + 1), Red, buffer, pal);
		put_pixel((3, height + 1), Maroon, buffer, pal);

		put_pixel((1, height + 2), Black, buffer, pal);
		put_pixel((2, height + 2), Maroon, buffer, pal);
		put_pixel((3, height + 2), Red, buffer, pal);

		put_pixel((2, height + 3), Black, buffer, pal);
		put_pixel((3, height + 3), Black, buffer, pal);

		for x in FRAME_BORDER..width {
			// top horizontal line
			put_pixel((x, 0), Black, buffer, pal);
			put_pixel((x, 1), Red, buffer, pal);
			put_pixel((x, 2), Black, buffer, pal);

			// bottom horizontal line
			put_pixel((x, height + 1), Black, buffer, pal);
			put_pixel((x, height + 2), Red, buffer, pal);
			put_pixel((x, height + 3), Black, buffer, pal);
		}

		for y in FRAME_BORDER..height {
			// left vertical line
			put_pixel((0, y), Black, buffer, pal);
			put_pixel((1, y), Red, buffer, pal);
			put_pixel((2, y), Black, buffer, pal);

			// right vertical line
			put_pixel((width + 1, y), Black, buffer, pal);
			put_pixel((width + 2, y), Red, buffer, pal);
			put_pixel((width + 3, y), Black, buffer, pal);
		}
	}

	fn width(&self) -> u32 {
		self.size.width
	}

	fn height(&self) -> u32 {
		self.size.height
	}
}

#[derive(Copy, Clone)]
enum FrameColor {
	Red = 0x20,
	Maroon = 0x40,
	Black = 0x60,
}

fn put_pixel((x, y): (u32, u32), color: FrameColor, buffer: &mut [u8], pal: &[u8]) {
	let buffer = &mut buffer[Point::xy(x, y).range()];

	buffer[0] = pal[color as usize * 3 + 0] << 2;
	buffer[1] = pal[color as usize * 3 + 1] << 2;
	buffer[2] = pal[color as usize * 3 + 2] << 2;
}
