use std::ops::GeneratorState;
use std::thread;
use std::time::{Duration, Instant};

use sdl2;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture};
use sdl2::event::Event;

use super::{Application, Config, Command};

mod input;
use input::convert;

const DEFAULT_DELAY: Duration = Duration::from_millis(10);

pub fn run(cfg: Config, mut app: impl Application) {
	let context = sdl2::init().unwrap();
	let video = context.video().unwrap();

	let window = video
		.window(cfg.title, cfg.width as u32 * 2, cfg.height as u32 * 2)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	canvas.set_scale(2.0, 2.0).unwrap();
	let creator = canvas.texture_creator();

	let mut events = context.event_pump().unwrap();

	let mut palette = Vec::new();
	let mut layer: Vec<(Texture, Rect)> = Vec::new();
	let mut state = app.start();

	'main: loop {
		let instant = Instant::now();

		handle_events(&mut app, &mut events);

		match state.as_mut().resume() {
			GeneratorState::Yielded(batch) => {
				for cmd in batch.commands {
					match cmd {
						Command::Palette(pal) => {
							palette = pal;
						}
						Command::Draw(drawable, point) => {
							let rect = Rect::new(point.x, point.y, drawable.width(), drawable.height());

							let mut texture = creator
								.create_texture_streaming(PixelFormatEnum::RGB24, rect.width(), rect.height())
								.unwrap();

							texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
								drawable.draw(buffer, pitch, &palette)
							}).unwrap();

							layer.push((texture, rect));
						}
						Command::Print(font, text, point) => {
							let rect = Rect::new(point.x, point.y, font.width(text), font.height(text));

							let mut texture = creator
								.create_texture_streaming(PixelFormatEnum::RGBA8888, rect.width(), rect.height())
								.unwrap();

							texture.set_blend_mode(BlendMode::Blend);
							texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
								font.print(buffer, pitch, &palette, text)
							}).unwrap();

							layer.push((texture, rect));
						}
						Command::Pop => {
							layer.pop();
						}
						Command::Clear => {
							layer.clear();
						}
						Command::Present => {
							for item in layer.iter() {
								canvas.copy(&item.0, None, item.1).unwrap();
							}

							canvas.present();
						}
						Command::FadeIn => {
							for item in layer.iter_mut() {
								item.0.set_blend_mode(BlendMode::Blend);
							}

							for step in 0u8..128u8 {
								canvas.clear();

								for item in layer.iter_mut() {
									item.0.set_alpha_mod(step * 2);

									canvas.copy(&item.0, None, item.1).unwrap();
								}

								canvas.present();

								thread::sleep(Duration::from_millis(3));
							}

							for item in layer.iter_mut() {
								item.0.set_blend_mode(BlendMode::None);
							}
						}
						Command::FadeOut => {
							let prev_mode = canvas.blend_mode();
							canvas.set_blend_mode(BlendMode::Blend);

							for step in 0u8..128u8 {
								canvas.set_draw_color(Color::RGBA(0, 0, 0, step));
								canvas.fill_rect(Rect::new(0, 0, cfg.width as u32, cfg.height as u32)).unwrap();
								canvas.present();

								thread::sleep(Duration::from_millis(3));
							}

							canvas.set_blend_mode(prev_mode);
						}
					}
				}

				let mut time_left = DEFAULT_DELAY;

				if let Some(duration) = batch.sleep {
					if duration > instant.elapsed() {
						time_left = duration - instant.elapsed();
					}
				}

				// TODO: get app state

				while time_left > Duration::from_millis(0) {
					handle_events(&mut app, &mut events);

					let delta = if time_left > DEFAULT_DELAY { DEFAULT_DELAY } else { time_left };
					time_left -= delta;

					thread::sleep(delta);

					// TODO: break if state has changed
				}
			}
			GeneratorState::Complete(()) => break 'main,
		}
	}
}

pub fn handle_events(app: &mut impl Application, events: &mut sdl2::EventPump) {
	for event in events.poll_iter() {
		match event {
			Event::Quit { .. } => app.stop(),
			Event::KeyDown { keycode: Some(key), .. } => {
				if let Some(key) = convert(key) {
					app.key_down(key);
				}
			},
			_ => {},
		}
	}
}
