// sub_CA67
pub fn unpack(data: &[u8]) -> Option<Vec<u8>> {
	let mut data = data.iter().cloned();
	let mut counter = data.next().unwrap();

	if counter == 0 {
		return None;
	}

	let mut table = [0; 1024];

	for i in 0u8..=255 {
		table[usize::from(i) << 2] = i;
	}

	let byte_5170 = data.next().unwrap();

	// loc_CABF
	while counter != 0 {
		let idx = usize::from(data.next().unwrap()) << 2;

		let byte_516d = usize::from(data.next().unwrap());
		let byte_bx = usize::from(data.next().unwrap());

		table[idx + 2] = byte_bx as u8;
		table[idx + 3] = if table[(byte_bx << 2) + 1] != 0 { 2 } else { 1 };

		table[idx + 0] = byte_516d as u8;
		table[idx + 1] = if table[(byte_516d << 2) + 1] != 0 {
			2
		} else {
			1
		};

		counter -= 1;
	}

	let mut unpacked = Vec::new();
	let mut case = 0;
	let mut word_516e = 0;

	let mut stack = Vec::new(); // (ah, al)

	// loc_CB14
	'main: while let Some(mut al) = data.next() {
		if al == byte_5170 {
			// loc_CB38
			switcher(
				&mut unpacked,
				&mut case,
				&mut word_516e,
				0,
				data.next().unwrap(),
			);

			if case < 0 {
				break 'main;
			}

			continue 'main;
		}

		// loc_CB1B
		'loc_cb1b: loop {
			let mut ah: u8;

			let idx = usize::from(al) << 2;

			al = table[idx + 0];
			ah = table[idx + 1];

			if ah == 0 {
				switcher(&mut unpacked, &mut case, &mut word_516e, ah, al);

				if case < 0 {
					break 'main;
				}

				continue 'main;
			}

			// loc_CB2A
			stack.push((table[idx + 3], table[idx + 2]));

			// loc_CB31
			while ah == 1 {
				switcher(&mut unpacked, &mut case, &mut word_516e, ah, al);

				if case < 0 {
					break 'main;
				}

				match stack.pop() {
					Some((h, l)) => {
						ah = h;
						al = l;
					}
					None => break 'loc_cb1b, // goto loc_CB14
				}
			}

			// goto loc_CB1B
		}
	}

	Some(unpacked)
}

fn switcher(unpacked: &mut Vec<u8>, case: &mut i32, word_516e: &mut u16, ah: u8, al: u8) {
	*case = match *case {
		// loc_CB43
		0 => {
			if al == 0 {
				-1 // goto loc_CB7D
			} else if al < 0x40 {
				*word_516e = u16::from_le_bytes([al, ah]) & 0x3F;
				1
			} else if al < 0x80 {
				*word_516e = u16::from_le_bytes([*word_516e as u8, al & 0x3F]);
				2
			} else if al < 0xC0 {
				*word_516e = u16::from_le_bytes([al, ah]) & 0x3F;
				3
			} else {
				*word_516e = u16::from_le_bytes([al & 0x3F, *word_516e as u8]);
				4
			}
		}
		// loc_CB8D
		1 => {
			unpacked.push(al);

			*word_516e -= 1;

			if *word_516e == 0 {
				//if unpacked.len() == 0xFFFF {
				//	return -1;		// goto CBB7
				//}

				0
			} else {
				1
			}
		}
		// loc_CB85
		2 => {
			*word_516e = (*word_516e & 0xFF00) | al as u16;
			1
		}
		// loc_CBA6
		3 => {
			let new_len = unpacked.len() + *word_516e as usize;
			unpacked.resize(new_len, al);

			//if unpacked.len() == 0xFFFF {
			//	return -1;		// goto CBB7
			//}

			0
		}
		// loc_CB9E
		4 => {
			*word_516e = u16::from_le_bytes([al, *word_516e as u8]);
			3
		}
		_ => {
			panic!("zip::switcher: case {} is not found", *case);
		}
	}
}
