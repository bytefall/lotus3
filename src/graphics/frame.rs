use super::{Point, Size};

pub const FRAME_BORDER: u32 = 4;

pub struct Frame {
    pub size: Size,
}

impl Frame {
    pub fn new(size: Size) -> Self {
        Self { size }
    }

    pub fn draw(&self, buffer: &mut [u32], pal: &[u8]) {
        let width = self.size.width - FRAME_BORDER; // inner width
        let height = self.size.height - FRAME_BORDER; // inner height

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

        put_pixel((width, 0), Black, buffer, pal);
        put_pixel((width + 1, 0), Black, buffer, pal);

        put_pixel((width, 1), Red, buffer, pal);
        put_pixel((width + 1, 1), Maroon, buffer, pal);
        put_pixel((width + 2, 1), Black, buffer, pal);

        put_pixel((width, 2), Maroon, buffer, pal);
        put_pixel((width + 1, 2), Red, buffer, pal);
        put_pixel((width + 2, 2), Maroon, buffer, pal);
        put_pixel((width + 3, 2), Black, buffer, pal);

        put_pixel((width, 3), Black, buffer, pal);
        put_pixel((width + 1, 3), Maroon, buffer, pal);
        put_pixel((width + 2, 3), Red, buffer, pal);
        put_pixel((width + 3, 3), Black, buffer, pal);

        // right bottom corner

        put_pixel((width, height), Black, buffer, pal);
        put_pixel((width + 1, height), Maroon, buffer, pal);
        put_pixel((width + 2, height), Red, buffer, pal);
        put_pixel((width + 3, height), Black, buffer, pal);

        put_pixel((width, height + 1), Maroon, buffer, pal);
        put_pixel((width + 1, height + 1), Red, buffer, pal);
        put_pixel((width + 2, height + 1), Maroon, buffer, pal);
        put_pixel((width + 3, height + 1), Black, buffer, pal);

        put_pixel((width, height + 2), Red, buffer, pal);
        put_pixel((width + 1, height + 2), Maroon, buffer, pal);
        put_pixel((width + 2, height + 2), Black, buffer, pal);

        put_pixel((width, height + 3), Black, buffer, pal);
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
}

#[derive(Copy, Clone)]
enum FrameColor {
    Red = 0x20,
    Maroon = 0x40,
    Black = 0x60,
}

fn put_pixel((x, y): (u32, u32), color: FrameColor, buffer: &mut [u32], pal: &[u8]) {
    let color = color as usize * 3;

    buffer[Point::xy(x, y).index()] = u32::from_be_bytes([
        255,
        pal[color] << 2,
        pal[color + 1] << 2,
        pal[color + 2] << 2,
    ]);
}
