use std::ops::{Generator, GeneratorState};
use std::time::Duration;

use crate::data::Archive;
use crate::graphics::{Point, Screen, Sprite};

pub enum Action<'a> {
	Show(&'a str, Duration),
	ShowSeries(Vec<&'a str>, Duration),
	Animate(&'a str, Duration, (u32, u32), Box<Fn(i32) -> (i32, i32)>),
	FadeOut,
}

pub struct Scene<'a> {
	actions: Vec<Action<'a>>,
}

impl<'a> Scene<'a> {
	pub fn from(actions: Vec<Action<'a>>) -> Scene<'a> {
		Scene { actions }
	}

	pub fn execute(
		&'a mut self,
		screen: &'a mut Screen,
		arc: &'a mut Archive,
	) -> impl Generator<Yield = Duration, Return = ()> + 'a {
		move || {
			for a in &self.actions {
				match a {
					Action::Show(key, delay) => {
						let (data, pal) = arc.get_with_palette(key).unwrap();

						screen.set_palette(pal);
						screen.add_sprite(Sprite::from(data), None);

						let mut gen = screen.fade_in();

						'fade_in: loop {
							match unsafe { gen.resume() } {
								GeneratorState::Yielded(duration) => yield duration,
								GeneratorState::Complete(()) => break 'fade_in,
							}
						}

						yield *delay
					}
					Action::ShowSeries(keys, delay) => {
						let (_, pal) = arc.get_with_palette(keys.last().unwrap()).unwrap();

						screen.set_palette(pal);

						for key in keys {
							screen.add_sprite(Sprite::from(arc.get(key).unwrap()), None);
							screen.update();
							screen.pop_sprite();

							yield *delay
						}
					}
					Action::Animate(key, delay, (width, height), pos) => {
						let list = arc.get_series(key, width * height).unwrap();

						for i in 0..list.len() {
							let data = list.get(i).unwrap();
							let (x, y) = pos(i as i32);

							screen.add_sprite(
								Sprite::from(data.to_vec()).with_size(*width, *height),
								Some(Point { x, y }),
							);
							screen.update();
							screen.pop_sprite();

							yield *delay
						}
					}
					Action::FadeOut => {
						let mut gen = screen.fade_out();

						'fade: loop {
							match unsafe { gen.resume() } {
								GeneratorState::Yielded(duration) => yield duration,
								GeneratorState::Complete(()) => break 'fade,
							}
						}
					}
				}
			}
		}
	}
}
