#[macro_use]
extern crate log;

mod data;
#[macro_use]
mod ecs;
mod game;
mod graphics;
mod systems;

use crate::{
	data::Archive,
	ecs::{
		context::{Context, ContextBuilder},
		system::System as _,
	},
	game::{
		options::Config,
		script::CommandSequence,
		state::{GameFlow, GameState},
	},
	systems::{
		game::ModelSelect,
		intro::{Intro, Protection},
		menu::{DefineMenu, MainMenu},
		Cache, Input, Script, Timer, Window, WindowConfig,
	},
};

fn main() -> Result<(), std::io::Error> {
	pretty_env_logger::init();

	let arc = Archive::open(&"lotus.dat")?;
	let cfg = Config::new();

	let mut ctx = ContextBuilder::new()
		.inject(WindowConfig {
			title: "Lotus III: The Ultimate Challenge",
			width: 320,
			height: 200,
		})
		.inject(arc)
		.inject_mut(cfg)
		.inject_mut(CommandSequence::new())
		.inject_mut(GameFlow::new(GameState::Protection(String::new())))
		.system(Timer::bind()).unwrap()
		.system(Window::bind()).unwrap()
		.system(Input::bind()).unwrap()
		.system(Cache::bind()).unwrap()
		// game systems
		.system(Protection::bind()).unwrap()
		.system(Intro::bind()).unwrap()
		.system(MainMenu::bind()).unwrap()
		.system(DefineMenu::bind()).unwrap()
		.system(ModelSelect::bind()).unwrap()
		// -end-
		.system(Script::bind()).unwrap()
		.build()
		.unwrap();

	ctx.run().unwrap();

	Ok(())
}
