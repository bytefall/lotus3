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
                for o in OP_CODES[*data.next().unwrap() as usize] {
                    match o {
                        Code::Skip(num) => offset += num,
                        Code::Draw(num) => {
                            for _ in 0..*num {
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

enum Code {
    Skip(usize),
    Draw(u8),
}

macro_rules! op {
    ($a:expr) => {
        &[Code::Draw($a)]
    };

    (+$a:expr) => {
        &[Code::Skip($a)]
    };

    ($a:expr, +$b:expr) => {
        &[Code::Draw($a), Code::Skip($b)]
    };

    (+$a:expr, $b:expr) => {
        &[Code::Skip($a), Code::Draw($b)]
    };

    ($a:expr, +$b:expr, $c:expr) => {
        &[Code::Draw($a), Code::Skip($b), Code::Draw($c)]
    };

    (+$a:expr, $b:expr, +$c:expr) => {
        &[Code::Skip($a), Code::Draw($b), Code::Skip($c)]
    };

    ($a:expr, +$b:expr, $c:expr, +$d:expr) => {
        &[
            Code::Draw($a),
            Code::Skip($b),
            Code::Draw($c),
            Code::Skip($d),
        ]
    };

    (+$a:expr, $b:expr, +$c:expr, $d:expr) => {
        &[
            Code::Skip($a),
            Code::Draw($b),
            Code::Skip($c),
            Code::Draw($d),
        ]
    };

    ($a:expr, +$b:expr, $c:expr, +$d:expr, $e:expr) => {
        &[
            Code::Draw($a),
            Code::Skip($b),
            Code::Draw($c),
            Code::Skip($d),
            Code::Draw($e),
        ]
    };

    (+$a:expr, $b:expr, +$c:expr, $d:expr, +$e:expr) => {
        &[
            Code::Skip($a),
            Code::Draw($b),
            Code::Skip($c),
            Code::Draw($d),
            Code::Skip($e),
        ]
    };

    ($a:expr, +$b:expr, $c:expr, +$d:expr, $e:expr, +$f:expr) => {
        &[
            Code::Draw($a),
            Code::Skip($b),
            Code::Draw($c),
            Code::Skip($d),
            Code::Draw($e),
            Code::Skip($f),
        ]
    };

    (+$a:expr, $b:expr, +$c:expr, $d:expr, +$e:expr, $f:expr) => {
        &[
            Code::Skip($a),
            Code::Draw($b),
            Code::Skip($c),
            Code::Draw($d),
            Code::Skip($e),
            Code::Draw($f),
        ]
    };

    ($a:expr, +$b:expr, $c:expr, +$d:expr, $e:expr, +$f:expr, $g:expr) => {
        &[
            Code::Draw($a),
            Code::Skip($b),
            Code::Draw($c),
            Code::Skip($d),
            Code::Draw($e),
            Code::Skip($f),
            Code::Draw($g),
        ]
    };

    (+$a:expr, $b:expr, +$c:expr, $d:expr, +$e:expr, $f:expr, +$g:expr) => {
        &[
            Code::Skip($a),
            Code::Draw($b),
            Code::Skip($c),
            Code::Draw($d),
            Code::Skip($e),
            Code::Draw($f),
            Code::Skip($g),
        ]
    };

    ($a:expr, +$b:expr, $c:expr, +$d:expr, $e:expr, +$f:expr, $g:expr, +$h:expr) => {
        &[
            Code::Draw($a),
            Code::Skip($b),
            Code::Draw($c),
            Code::Skip($d),
            Code::Draw($e),
            Code::Skip($f),
            Code::Draw($g),
            Code::Skip($h),
        ]
    };

    (+$a:expr, $b:expr, +$c:expr, $d:expr, +$e:expr, $f:expr, +$g:expr, $h:expr) => {
        &[
            Code::Skip($a),
            Code::Draw($b),
            Code::Skip($c),
            Code::Draw($d),
            Code::Skip($e),
            Code::Draw($f),
            Code::Skip($g),
            Code::Draw($h),
        ]
    };
}

const OP_CODES: &[&[Code]] = &[
    op!(+8),
    op!(+7, 1),
    op!(+6, 1, +1),
    op!(+6, 2),
    op!(+5, 1, +2),
    op!(+5, 1, +1, 1),
    op!(+5, 2, +1),
    op!(+5, 3),
    op!(+4, 1, +3),
    op!(+4, 1, +2, 1),
    op!(+4, 1, +1, 1, +1),
    op!(+4, 1, +1, 2),
    op!(+4, 2, +2),
    op!(+4, 2, +1, 1),
    op!(+4, 3, +1),
    op!(+4, 4),
    op!(+3, 1, +4),
    op!(+3, 1, +3, 1),
    op!(+3, 1, +2, 1, +1),
    op!(+3, 1, +2, 2),
    op!(+3, 1, +1, 1, +2),
    op!(+3, 1, +1, 1, +1, 1),
    op!(+3, 1, +1, 2, +1),
    op!(+3, 1, +1, 3),
    op!(+3, 2, +3),
    op!(+3, 2, +2, 1),
    op!(+3, 2, +1, 1, +1),
    op!(+3, 2, +1, 2),
    op!(+3, 3, +2),
    op!(+3, 3, +1, 1),
    op!(+3, 4, +1),
    op!(+3, 5),
    op!(+2, 1, +5),
    op!(+2, 1, +4, 1),
    op!(+2, 1, +3, 1, +1),
    op!(+2, 1, +3, 2),
    op!(+2, 1, +2, 1, +2),
    op!(+2, 1, +2, 1, +1, 1),
    op!(+2, 1, +2, 2, +1),
    op!(+2, 1, +2, 3),
    op!(+2, 1, +1, 1, +3),
    op!(+2, 1, +1, 1, +2, 1),
    op!(+2, 1, +1, 1, +1, 1, +1),
    op!(+2, 1, +1, 1, +1, 2),
    op!(+2, 1, +1, 2, +2),
    op!(+2, 1, +1, 2, +1, 1),
    op!(+2, 1, +1, 3, +1),
    op!(+2, 1, +1, 4),
    op!(+2, 2, +4),
    op!(+2, 2, +3, 1),
    op!(+2, 2, +2, 1, +1),
    op!(+2, 2, +2, 2),
    op!(+2, 2, +1, 1, +2),
    op!(+2, 2, +1, 1, +1, 1),
    op!(+2, 2, +1, 2, +1),
    op!(+2, 2, +1, 3),
    op!(+2, 3, +3),
    op!(+2, 3, +2, 1),
    op!(+2, 3, +1, 1, +1),
    op!(+2, 3, +1, 2),
    op!(+2, 4, +2),
    op!(+2, 4, +1, 1),
    op!(+2, 5, +1),
    op!(+2, 6),
    op!(+1, 1, +6),
    op!(+1, 1, +5, 1),
    op!(+1, 1, +4, 1, +1),
    op!(+1, 1, +4, 2),
    op!(+1, 1, +3, 1, +2),
    op!(+1, 1, +3, 1, +1, 1),
    op!(+1, 1, +3, 2, +1),
    op!(+1, 1, +3, 3),
    op!(+1, 1, +2, 1, +3),
    op!(+1, 1, +2, 1, +2, 1),
    op!(+1, 1, +2, 1, +1, 1, +1),
    op!(+1, 1, +2, 1, +1, 2),
    op!(+1, 1, +2, 2, +2),
    op!(+1, 1, +2, 2, +1, 1),
    op!(+1, 1, +2, 3, +1),
    op!(+1, 1, +2, 4),
    op!(+1, 1, +1, 1, +4),
    op!(+1, 1, +1, 1, +3, 1),
    op!(+1, 1, +1, 1, +2, 1, +1),
    op!(+1, 1, +1, 1, +2, 2),
    op!(+1, 1, +1, 1, +1, 1, +2),
    op!(+1, 1, +1, 1, +1, 1, +1, 1),
    op!(+1, 1, +1, 1, +1, 2, +1),
    op!(+1, 1, +1, 1, +1, 3),
    op!(+1, 1, +1, 2, +3),
    op!(+1, 1, +1, 2, +2, 1),
    op!(+1, 1, +1, 2, +1, 1, +1),
    op!(+1, 1, +1, 2, +1, 2),
    op!(+1, 1, +1, 3, +2),
    op!(+1, 1, +1, 3, +1, 1),
    op!(+1, 1, +1, 4, +1),
    op!(+1, 1, +1, 5),
    op!(+1, 2, +5),
    op!(+1, 2, +4, 1),
    op!(+1, 2, +3, 1, +1),
    op!(+1, 2, +3, 2),
    op!(+1, 2, +2, 1, +2),
    op!(+1, 2, +2, 1, +1, 1),
    op!(+1, 2, +2, 2, +1),
    op!(+1, 2, +2, 3),
    op!(+1, 2, +1, 1, +3),
    op!(+1, 2, +1, 1, +2, 1),
    op!(+1, 2, +1, 1, +1, 1, +1),
    op!(+1, 2, +1, 1, +1, 2),
    op!(+1, 2, +1, 2, +2),
    op!(+1, 2, +1, 2, +1, 1),
    op!(+1, 2, +1, 3, +1),
    op!(+1, 2, +1, 4),
    op!(+1, 3, +4),
    op!(+1, 3, +3, 1),
    op!(+1, 3, +2, 1, +1),
    op!(+1, 3, +2, 2),
    op!(+1, 3, +1, 1, +2),
    op!(+1, 3, +1, 1, +1, 1),
    op!(+1, 3, +1, 2, +1),
    op!(+1, 3, +1, 3),
    op!(+1, 4, +3),
    op!(+1, 4, +2, 1),
    op!(+1, 4, +1, 1, +1),
    op!(+1, 4, +1, 2),
    op!(+1, 5, +2),
    op!(+1, 5, +1, 1),
    op!(+1, 6, +1),
    op!(+1, 7),
    op!(1, +7),
    op!(1, +6, 1),
    op!(1, +5, 1, +1),
    op!(1, +5, 2),
    op!(1, +4, 1, +2),
    op!(1, +4, 1, +1, 1),
    op!(1, +4, 2, +1),
    op!(1, +4, 3),
    op!(1, +3, 1, +3),
    op!(1, +3, 1, +2, 1),
    op!(1, +3, 1, +1, 1, +1),
    op!(1, +3, 1, +1, 2),
    op!(1, +3, 2, +2),
    op!(1, +3, 2, +1, 1),
    op!(1, +3, 3, +1),
    op!(1, +3, 4),
    op!(1, +2, 1, +4),
    op!(1, +2, 1, +3, 1),
    op!(1, +2, 1, +2, 1, +1),
    op!(1, +2, 1, +2, 2),
    op!(1, +2, 1, +1, 1, +2),
    op!(1, +2, 1, +1, 1, +1, 1),
    op!(1, +2, 1, +1, 2, +1),
    op!(1, +2, 1, +1, 3),
    op!(1, +2, 2, +3),
    op!(1, +2, 2, +2, 1),
    op!(1, +2, 2, +1, 1, +1),
    op!(1, +2, 2, +1, 2),
    op!(1, +2, 3, +2),
    op!(1, +2, 3, +1, 1),
    op!(1, +2, 4, +1),
    op!(1, +2, 5),
    op!(1, +1, 1, +5),
    op!(1, +1, 1, +4, 1),
    op!(1, +1, 1, +3, 1, +1),
    op!(1, +1, 1, +3, 2),
    op!(1, +1, 1, +2, 1, +2),
    op!(1, +1, 1, +2, 1, +1, 1),
    op!(1, +1, 1, +2, 2, +1),
    op!(1, +1, 1, +2, 3),
    op!(1, +1, 1, +1, 1, +3),
    op!(1, +1, 1, +1, 1, +2, 1),
    op!(1, +1, 1, +1, 1, +1, 1, +1),
    op!(1, +1, 1, +1, 1, +1, 2),
    op!(1, +1, 1, +1, 2, +2),
    op!(1, +1, 1, +1, 2, +1, 1),
    op!(1, +1, 1, +1, 3, +1),
    op!(1, +1, 1, +1, 4),
    op!(1, +1, 2, +4),
    op!(1, +1, 2, +3, 1),
    op!(1, +1, 2, +2, 1, +1),
    op!(1, +1, 2, +2, 2),
    op!(1, +1, 2, +1, 1, +2),
    op!(1, +1, 2, +1, 1, +1, 1),
    op!(1, +1, 2, +1, 2, +1),
    op!(1, +1, 2, +1, 3),
    op!(1, +1, 3, +3),
    op!(1, +1, 3, +2, 1),
    op!(1, +1, 3, +1, 1, +1),
    op!(1, +1, 3, +1, 2),
    op!(1, +1, 4, +2),
    op!(1, +1, 4, +1, 1),
    op!(1, +1, 5, +1),
    op!(1, +1, 6),
    op!(2, +6),
    op!(2, +5, 1),
    op!(2, +4, 1, +1),
    op!(2, +4, 2),
    op!(2, +3, 1, +2),
    op!(2, +3, 1, +1, 1),
    op!(2, +3, 2, +1),
    op!(2, +3, 3),
    op!(2, +2, 1, +3),
    op!(2, +2, 1, +2, 1),
    op!(2, +2, 1, +1, 1, +1),
    op!(2, +2, 1, +1, 2),
    op!(2, +2, 2, +2),
    op!(2, +2, 2, +1, 1),
    op!(2, +2, 3, +1),
    op!(2, +2, 4),
    op!(2, +1, 1, +4),
    op!(2, +1, 1, +3, 1),
    op!(2, +1, 1, +2, 1, +1),
    op!(2, +1, 1, +2, 2),
    op!(2, +1, 1, +1, 1, +2),
    op!(2, +1, 1, +1, 1, +1, 1),
    op!(2, +1, 1, +1, 2, +1),
    op!(2, +1, 1, +1, 3),
    op!(2, +1, 2, +3),
    op!(2, +1, 2, +2, 1),
    op!(2, +1, 2, +1, 1, +1),
    op!(2, +1, 2, +1, 2),
    op!(2, +1, 3, +2),
    op!(2, +1, 3, +1, 1),
    op!(2, +1, 4, +1),
    op!(2, +1, 5),
    op!(3, +5),
    op!(3, +4, 1),
    op!(3, +3, 1, +1),
    op!(3, +3, 2),
    op!(3, +2, 1, +2),
    op!(3, +2, 1, +1, 1),
    op!(3, +2, 2, +1),
    op!(3, +2, 3),
    op!(3, +1, 1, +3),
    op!(3, +1, 1, +2, 1),
    op!(3, +1, 1, +1, 1, +1),
    op!(3, +1, 1, +1, 2),
    op!(3, +1, 2, +2),
    op!(3, +1, 2, +1, 1),
    op!(3, +1, 3, +1),
    op!(3, +1, 4),
    op!(4, +4),
    op!(4, +3, 1),
    op!(4, +2, 1, +1),
    op!(4, +2, 2),
    op!(4, +1, 1, +2),
    op!(4, +1, 1, +1, 1),
    op!(4, +1, 2, +1),
    op!(4, +1, 3),
    op!(5, +3),
    op!(5, +2, 1),
    op!(5, +1, 1, +1),
    op!(5, +1, 2),
    op!(6, +2),
    op!(6, +1, 1),
    op!(7, +1),
    op!(8),
];
