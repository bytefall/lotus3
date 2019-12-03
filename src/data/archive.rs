use super::zip;
use std::{
	cell::RefCell,
	collections::HashMap,
	fs::File,
	io::{prelude::*, BufReader, Error, SeekFrom},
	path::Path,
	slice::from_raw_parts,
	str::from_utf8,
};

const TABLE_OFFSET: u64 = 0xC;

struct Item {
	offset: u32,
	length: u32,
}

pub struct Archive {
	file: RefCell<BufReader<File>>,
	items: HashMap<String, Item>,
}

impl Archive {
	pub fn open(path: &dyn AsRef<Path>) -> Result<Archive, Error> {
		let mut file = BufReader::new(File::open(path)?);
		file.seek(SeekFrom::Start(TABLE_OFFSET)).unwrap();

		let mut items = HashMap::new();
		let mut prev_key = String::new();

		loop {
			let mut buffer = [0; 10];
			file.read(&mut buffer).unwrap();

			if buffer[0] == 0x0 {
				break;
			}

			let key = (unsafe { from_utf8(from_raw_parts(&buffer[0], 3)) }).unwrap();
			let offset = (((buffer[9] as u32) << 8) | (buffer[8] as u32)) << 9;

			items.entry(key.to_string()).or_insert(Item { offset, length: 0 });

			if let Some(prev) = items.get_mut(&prev_key) {
				(*prev).length = offset - (*prev).offset;
			}

			prev_key = key.to_string();
		}

		Ok(Self {
			file: RefCell::new(file),
			items,
		})
	}

	pub fn get(&self, key: &str) -> Option<Vec<u8>> {
		let item = self.items.get(&key.to_string())?;

		let mut buffer = Vec::with_capacity(item.length as usize);
		buffer.resize(item.length as usize, 0);

		let mut file = self.file.borrow_mut();
		file.seek(SeekFrom::Start(item.offset.into())).unwrap();
		file.read_exact(&mut buffer).unwrap();

		zip::unpack(&buffer)
	}

	pub fn get_with_palette(&self, key: &str) -> Option<(Vec<u8>, Vec<u8>)> {
		let mut data = self.get(key)?;

		let len = data.len();
		let pal = data.split_off(len - 256 * 3);

		Some((data, pal))
	}

	pub fn get_series(&self, key: &str, size: u32) -> Option<Vec<Vec<u8>>> {
		let mut data = self.get(key)?;
		let mut rest;

		let size = size as usize;
		let mut series = vec![];

		for _ in 0..(data.len() / size) {
			rest = data.split_off(size);

			series.push(data);

			data = rest;
		}

		Some(series)
	}
}
