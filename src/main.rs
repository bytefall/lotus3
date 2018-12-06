#![feature(nll)]
#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

mod data;
mod graphics;
mod intro;

use crate::data::Archive;
use crate::graphics::Screen;
use crate::intro::{Action, Scene};

fn main() -> Result<(), std::io::Error> {
	let mut arc = Archive::open(&PathBuf::from("lotus.dat"))?;
	let mut screen = Screen::new();

	let mut scene = Scene::from(vec![
		// protection screen
		Action::Show("I21", Duration::from_millis(200)),
		Action::Print("ENTER CODE FOR WINDOW 47", (60, 140)),
		Action::Sleep(Duration::from_millis(1000)),
		// Gremlin Presents
		Action::Show("Q00", Duration::from_millis(200)),
		Action::Animate(
			"Q01",
			Duration::from_millis(200),
			(16, 8),
			Box::new(|step| if step < 4 { (112, 85) } else { (144, 110) }),
		),
		Action::FadeOut,
		// A Magnetic Fields Production
		Action::ShowSeries(
			vec![
				"Q02", "Q03", "Q04", "Q05", "Q06", "Q07", "Q08", "Q09", "Q0A", "Q0B", "Q0C", "Q0D",
				"Q0E", "Q0F", "Q10", "Q11", "Q12", "Q13", "Q14", "Q15", "Q16", "Q17",
			],
			Duration::from_millis(50),
		),
	]);

	let mut gen = scene.execute(&mut screen, &mut arc);

	'main: loop {
		let instant = Instant::now();

		match unsafe { gen.resume() } {
			GeneratorState::Yielded(duration) => {
				if duration > instant.elapsed() {
					thread::sleep(duration - instant.elapsed());
				}
			}
			GeneratorState::Complete(()) => break 'main,
		}
	}

	thread::sleep(Duration::from_millis(1000));

	Ok(())
}
