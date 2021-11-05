#![allow(clippy::identity_op)]

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
		game::{AudioTuner, ModelSelect},
		intro::{Intro, Protection},
		menu::{DefineMenu, MainMenu},
		Cache, Input, Script, Timer, Window, WindowConfig,
	},
};

mod data;
#[macro_use]
mod ecs;
mod game;
mod graphics;
mod systems;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

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
		.system(Timer::bind())?
		.system(Window::bind())?
		.system(Input::bind())?
		.system(Cache::bind())?
		// game systems
		.system(Protection::bind())?
		.system(Intro::bind())?
		.system(MainMenu::bind())?
		.system(DefineMenu::bind())?
		.system(ModelSelect::bind())?
		.system(AudioTuner::bind())?
		// -end-
		.system(Script::bind())?
		.build()?;

	ctx.run()?;

	Ok(())
}
