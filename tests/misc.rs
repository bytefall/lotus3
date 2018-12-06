use std::path::PathBuf;

use lotus3::data::Archive;

pub fn get_data(key: &str) -> Vec<u8> {
	Archive::open(&PathBuf::from("lotus.dat")).unwrap().get(key).unwrap()
}
