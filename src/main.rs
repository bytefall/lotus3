#![feature(nll)]
#![feature(generators, generator_trait)]

use std::path::PathBuf;

mod app;
mod data;
#[macro_use] mod graphics;
mod lotus;

use crate::app::Config;
use crate::graphics::{WIDTH, HEIGHT};
use crate::data::Archive;
use crate::lotus::Lotus;

fn main() -> Result<(), std::io::Error> {
	let cfg = Config {
		title: "Lotus III: The Ultimate Challenge",
		width: WIDTH,
		height: HEIGHT,
	};

	let arc = Archive::open(&PathBuf::from("lotus.dat"))?;

	app::run(cfg, Lotus::new(arc));

	Ok(())
}
