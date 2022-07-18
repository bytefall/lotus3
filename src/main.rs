#![allow(clippy::identity_op)]

use crate::{app::Application, data::Archive, engine::GameEngine, game::options::Config};

mod app;
mod data;
mod engine;
mod game;
mod graphics;
mod screen;
mod systems;
mod task;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let arc = Archive::open(&lotus3::ARCHIVE_FILE_NAME)?;
	let cfg = Config::new();
	let app = Application::new("Lotus III: The Ultimate Challenge")?;

	let mut game = GameEngine::new(arc, cfg, app.get_input(), app.get_screen()?, game::main)?;

	app.run(move |ctx, signal| game.step(ctx, signal));
}
