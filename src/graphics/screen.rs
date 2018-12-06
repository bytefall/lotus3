use std::ops::Generator;
use std::time::Duration;

use sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use super::{Font, Point, Sprite, HEIGHT, WIDTH};

pub struct Screen {
	canvas: Canvas<Window>,
	creator: TextureCreator<WindowContext>,
	items: Vec<(Texture, Rect)>,
	palette: Vec<u8>,
}

impl Screen {
	pub fn new() -> Screen {
		let context = sdl2::init().unwrap();
		let video = context.video().unwrap();

		let window = video
			.window("Lotus III: The Ultimate Challenge", WIDTH as u32 * 2, HEIGHT as u32 * 2)
			.position_centered()
			.build()
			.unwrap();

		let mut canvas = window.into_canvas().build().unwrap();
		canvas.set_scale(2.0, 2.0).unwrap();

		let creator = canvas.texture_creator();

		Screen {
			canvas,
			creator,
			items: vec![],
			palette: vec![],
		}
	}

	pub fn add_sprite(&mut self, sprite: Sprite, point: Option<Point>) {
		let mut texture = self
			.creator
			.create_texture_streaming(
				PixelFormatEnum::RGB24,
				sprite.size.width,
				sprite.size.height,
			)
			.unwrap();

		texture.with_lock(None, sprite.draw(&self.palette)).unwrap();

		let mut x = 0;
		let mut y = 0;

		if let Some(point) = point {
			x = point.x;
			y = point.y;
		}

		self.items.push((
			texture,
			Rect::new(x, y, sprite.size.width, sprite.size.height),
		));
	}

	pub fn add_text(&mut self, text: &str, font: &Font, point: Point) {
		let size = font.size(text);

		let mut texture = self
			.creator
			.create_texture_streaming(
				PixelFormatEnum::RGBA8888,
				size.width,
				size.height,
			)
			.unwrap();

		texture.set_blend_mode(BlendMode::Blend);
		texture.with_lock(None, font.print(&self.palette, text)).unwrap();

		self.items.push((
			texture,
			Rect::new(point.x, point.y, size.width, size.height),
		));
	}

	pub fn pop_sprite(&mut self) {
		self.items.pop();
	}

	pub fn set_palette(&mut self, palette: Vec<u8>) {
		self.palette = palette;
	}

	pub fn update(&mut self) {
		self.canvas.clear();

		for item in self.items.iter() {
			self.canvas.copy(&item.0, None, item.1).unwrap();
		}

		self.canvas.present();
	}

	pub fn fade_in<'a>(&'a mut self) -> impl Generator<Yield = Duration, Return = ()> + 'a {
		move || {
			for item in self.items.iter_mut() {
				item.0.set_blend_mode(BlendMode::Blend);
			}

			for step in 0u8..128u8 {
				for item in self.items.iter_mut() {
					item.0.set_alpha_mod(step * 2);
				}

				self.update();

				yield Duration::from_millis(3)
			}

			for item in self.items.iter_mut() {
				item.0.set_blend_mode(BlendMode::None);
			}
		}
	}

	pub fn fade_out<'a>(&'a mut self) -> impl Generator<Yield = Duration, Return = ()> + 'a {
		move || {
			for _ in 1u8..8u8 {
				for item in self.items.iter_mut() {
					item.0
						.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
							for offset in 0..buffer.len() {
								buffer[offset] = buffer[offset] >> 1;
							}
						})
						.unwrap();

					self.canvas.copy(&item.0, None, item.1).unwrap();
				}

				self.canvas.present();

				yield Duration::from_millis(100)
			}
		}
	}
}
