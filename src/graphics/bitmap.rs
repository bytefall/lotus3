use crate::graphics::{Point, SCREEN_WIDTH};

pub struct Bitmap {
    data: Vec<u8>,
}

impl Bitmap {
    pub fn from(data: Vec<u8>, par1: u8, par2: u8) -> Self {
        Self {
            data: decode(data, par1, par2),
        }
    }

    pub fn draw(&self, index: usize, point: Point, buffer: &mut [u32], palette: &[u8]) {
        let pos = index << 3; // 8-byte table

        let op_pos = u16::from_le_bytes([self.data[pos], self.data[pos + 1]]) << 4;
        let repeat = (u16::from_le_bytes([self.data[pos + 2], self.data[pos + 3]]) + 7) >> 3;
        let height = u16::from_le_bytes([self.data[pos + 4], self.data[pos + 5]]) as usize;

        let mut data = self.data.iter().skip(op_pos as usize);

        let xx = point.x as usize;
        let yy = point.y as usize;

        for y in yy..yy + height {
            let mut offset = y * SCREEN_WIDTH as usize + xx;

            for _ in 0..repeat {
                for o in OP_CODES[*data.next().unwrap() as usize]
                    .into_iter()
                    .flatten()
                {
                    match o {
                        Code::Skip(num) => offset += num,
                        Code::Draw(num) => {
                            for _ in 0..num {
                                let val = *data.next().unwrap() as usize * 3;

                                buffer[offset] = u32::from_be_bytes([
                                    255,
                                    palette[val] << 2,
                                    palette[val + 1] << 2,
                                    palette[val + 2] << 2,
                                ]);

                                offset += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

const VAR_30A7: [u8; 94] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x14, 0x14, 0x14, 0x14,
    0x14, 0x1E, 0x1E, 0x1E, 0x1E, 0x1E, 0x28, 0x28, 0x28, 0x32, 0x32, 0x32, 0x3C, 0x3C, 0x3C, 0x46,
    0x46, 0x46, 0x50, 0x50, 0x50, 0x5A, 0x5A, 0x5A, 0x64, 0x64, 0x64, 0x6E, 0x6E, 0x6E, 0x78, 0x78,
    0x78, 0x82, 0x82, 0x82, 0x8C, 0x8C, 0x8C, 0x96, 0x96, 0x96, 0xA0, 0xA0, 0xA0, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x07, 0xFF,
];

pub fn decode(mut data: Vec<u8>, word_2e78_ah: u8, word_2e78_al: u8) -> Vec<u8> {
    let mut buffer = Vec::new();

    // load first part of 8-byte chunks (last chunk ends with 0xFF)
    for c in data.chunks(8) {
        buffer.extend_from_slice(c);

        if c.last() == Some(&0xFF) {
            break;
        }
    }

    if buffer.len() & 8 != 0 {
        for v in data.iter().skip(buffer.len()).take(8) {
            buffer.push(*v);
        }
    }

    let mut bx = 0;

    // process second part
    'loop1: loop {
        let tmp = buffer.len() >> 4;
        buffer[bx] = tmp as u8;
        buffer[bx + 1] = (tmp >> 8) as u8;

        let mut byte_c5e2 = word_2e78_al;

        if word_2e78_ah != 0 {
            byte_c5e2 += VAR_30A7[bx >> 3];
        }

        let mut si = u16::from_le_bytes([data[bx], data[bx + 1]]) << 4;
        let mut first = true;

        'loop2: loop {
            let mut dx = u16::from_le_bytes([data[bx + 2], data[bx + 3]]);

            'loop3: loop {
                let cx = if dx < 8 { dx } else { 8 };
                dx -= cx;

                let buf_len = buffer.len();

                let mut dh = 0;
                let mut ah = 0x80;

                buffer.push(0);

                for _ in 0..cx {
                    let mut al = data[si as usize];
                    si += 1;

                    if first {
                        si -= 1;
                        al >>= 4;
                    }

                    first = !first;

                    al &= 0xF;

                    if al != data[bx + 6] {
                        al += byte_c5e2;

                        buffer.push(al);

                        dh |= ah;
                    }

                    ah >>= 1;
                }

                buffer[buf_len] = dh;

                if dx == 0 {
                    break 'loop3;
                }
            }

            let mut idx = u16::from_le_bytes([data[bx + 4], data[bx + 5]]);
            idx -= 1;
            data[bx + 4] = idx as u8;
            data[bx + 5] = (idx >> 8) as u8;

            if idx == 0 {
                break 'loop2;
            }
        }

        // align buffer to 16-bytes boundary
        if buffer.len() % 16 > 0 {
            buffer.resize(buffer.len() + 16 - buffer.len() % 16, 0);
        }

        bx += 8;

        if data[bx - 1] == 0xFF {
            break 'loop1;
        }
    }

    buffer
}

#[derive(Copy, Clone)]
enum Code {
    Skip(usize),
    Draw(u8),
}

const OP_CODES: [[Option<Code>; 8]; 256] = create_table::<8, 256>();

const fn create_table<const X: usize, const Y: usize>() -> [[Option<Code>; X]; Y] {
    let mut codes = [[None; X]; Y];
    let mut y = 0;

    loop {
        let mut i = 0;
        let mut b = y;
        let mut x = 0;

        loop {
            let cf = b & 0x80 != 0;
            b <<= 1;

            if cf {
                // loc_DB7C
                match codes[y][x] {
                    Some(Code::Draw(v)) => codes[y][x] = Some(Code::Draw(v + 1)),
                    Some(Code::Skip(_)) => {
                        x += 1;
                        codes[y][x] = Some(Code::Draw(1));
                    }
                    None => codes[y][x] = Some(Code::Draw(1)),
                }
            } else {
                // loc_DB65
                match codes[y][x] {
                    Some(Code::Skip(v)) => codes[y][x] = Some(Code::Skip(v + 1)),
                    Some(Code::Draw(_)) => {
                        x += 1;
                        codes[y][x] = Some(Code::Skip(1));
                    }
                    None => codes[y][x] = Some(Code::Skip(1)),
                }
            }

            i += 1;

            if i == X {
                break;
            }
        }

        y += 1;

        if y == Y {
            break;
        }
    }

    codes
}
