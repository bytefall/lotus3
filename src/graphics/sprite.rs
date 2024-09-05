use super::{Point, Size, SCREEN_SIZE};

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

    pub fn draw(&self, buffer: &mut [u32], palette: &[u8]) {
        let mut src = self.data.iter();

        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let src = *src.next().unwrap() as usize * 3;

                buffer[Point::xy(x, y).index()] = u32::from_be_bytes([
                    255,
                    palette[src] << 2,
                    palette[src + 1] << 2,
                    palette[src + 2] << 2,
                ]);
            }
        }
    }
}
