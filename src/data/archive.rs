use eyre::{eyre, WrapErr, Result};
use std::{
	cell::RefCell,
	collections::HashMap,
	fs::File,
	io::{prelude::*, BufReader, SeekFrom},
	path::Path,
	slice::from_raw_parts,
	str::from_utf8,
};

use super::zip;

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
	pub fn open(path: &dyn AsRef<Path>) -> Result<Archive> {
		let file = File::open(path)
			.wrap_err_with(|| format!("Failed to open data file \"{}\"!", path.as_ref().to_str().unwrap_or("")))?;

		let mut file = BufReader::new(file);
		file.seek(SeekFrom::Start(TABLE_OFFSET))?;

		let mut items = HashMap::new();
		let mut prev_key = String::new();

		loop {
			let mut buffer = [0; 10];
			file.read(&mut buffer)?;

			if buffer[0] == 0 {
				break;
			}

			let key = (unsafe { from_utf8(from_raw_parts(&buffer[0], 3)) })?;
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

	pub fn get(&self, key: &str) -> Result<Vec<u8>> {
		let item = self.items.get(&key.to_string())
			.ok_or_else(|| eyre!("Item \"{}\" is not found in the data file!", key))?;

		let mut buffer = Vec::with_capacity(item.length as usize);
		buffer.resize(item.length as usize, 0);

		let mut file = self.file.borrow_mut();
		file.seek(SeekFrom::Start(item.offset.into()))?;
		file.read_exact(&mut buffer)?;

		zip::unpack(&buffer).ok_or_else(|| eyre!("Failed to unpack item \"{}\"!", key))
	}

	pub fn get_with_palette(&self, key: &str) -> Result<(Vec<u8>, Vec<u8>)> {
		let mut data = self.get(key)?;

		let len = data.len();
		let pal = data.split_off(len - 256 * 3);

		Ok((data, pal))
	}

	pub fn get_series(&self, key: &str, size: u32) -> Result<Vec<Vec<u8>>> {
		let mut data = self.get(key)?;
		let mut rest;

		let size = size as usize;
		let mut series = vec![];

		for _ in 0..(data.len() / size) {
			rest = data.split_off(size);

			series.push(data);

			data = rest;
		}

		Ok(series)
	}
}
