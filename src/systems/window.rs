use eyre::Result;
use generational_arena::{Arena, Index};
use sdl2::{
	pixels::{Color, PixelFormatEnum},
	rect::{Point as Point2, Rect},
	render::{BlendMode, Canvas, RenderTarget, Texture, TextureCreator},
	video::WindowContext,
	Sdl,
};
use std::{
	cmp::{max, min},
	thread,
	time::{Duration, Instant},
};

use crate::{
	ecs::system::System,
	graphics::{Drawable, PaintCanvas, Point, Printable, Size},
};

impl<T> PaintCanvas for Canvas<T>
where
	T: RenderTarget,
{
	fn color(&mut self, r: u8, g: u8, b: u8, a: u8) {
		self.set_draw_color(Color::RGBA(r, g, b, a));
	}

	fn point(&mut self, point: Point) {
		self.draw_point((point.x, point.y)).unwrap();
	}

	fn line(&mut self, start: Point, end: Point) {
		self.draw_line((start.x, start.y), (end.x, end.y)).unwrap();
	}
}

pub struct WindowConfig {
	pub title: &'static str,
	pub width: usize,
	pub height: usize,
}

pub struct Window {
	pub palette: Vec<u8>,
	pub context: Sdl,
	canvas: Canvas<sdl2::video::Window>,
	creator: TextureCreator<WindowContext>,
	cache: Arena<CacheItem>,
	screen: Vec<ScreenItem>,
}

struct CacheItem {
	txt: Texture,
	size: Size,
}

struct ScreenItem {
	id: Index,
	pos: Point,
}

pub struct IndexChain<'a> {
	win: &'a mut Window,
	pub id: Index,
}

impl<'a> IndexChain<'a> {
	pub fn show(&'a mut self, pos: Point) -> &'a mut Self {
		self.win.show(self.id, pos);
		self
	}
}

const PIXEL_FORMAT_RGB: PixelFormatEnum = PixelFormatEnum::RGB24;
const PIXEL_FORMAT_RGBA: PixelFormatEnum = PixelFormatEnum::RGBA8888;
const FADE_STEP: Duration = Duration::from_millis(3);

impl Window {
	pub fn draw(&mut self, sprite: &dyn Drawable) -> IndexChain {
		let size = Size::wh(sprite.width(), sprite.height());

		let mut txt = self.creator
			.create_texture_streaming(PIXEL_FORMAT_RGB, size.width, size.height)
			.unwrap();

		txt.with_lock(None, |buffer: &mut [u8], pitch: usize| {
			sprite.draw(buffer, pitch, &self.palette)
		}).unwrap();

		IndexChain {
			id: self.cache.insert(CacheItem { txt, size }),
			win: self,
		}
	}

	pub fn print(&mut self, font: &dyn Printable, text: &str) -> IndexChain {
		let size = Size::wh(font.width(&text), font.height(&text));

		let mut txt = self.creator
			.create_texture_streaming(PIXEL_FORMAT_RGBA, size.width, size.height)
			.unwrap();

		txt.set_blend_mode(BlendMode::Blend);
		txt.with_lock(None, |buffer: &mut [u8], pitch: usize| {
			font.print(buffer, pitch, &self.palette, &text)
		}).unwrap();

		IndexChain {
			id: self.cache.insert(CacheItem { txt, size }),
			win: self,
		}
	}

	pub fn paint<F>(&mut self, size: Size, foo: F) -> IndexChain
	where
		for<'r> F: FnOnce(&'r [u8], &'r mut dyn PaintCanvas),
	{
		let mut txt = self.creator
			.create_texture_target(PIXEL_FORMAT_RGBA, size.width, size.height)
			.unwrap();

		txt.set_blend_mode(BlendMode::Blend);

		let pal = &self.palette;

		self.canvas.with_texture_canvas(&mut txt, |tc| foo(pal, tc)).unwrap();

		IndexChain {
			id: self.cache.insert(CacheItem { txt, size }),
			win: self,
		}
	}

	/// Gets the texture size by its index.
	pub fn txt_size(&self, id: Index) -> Option<&Size> {
		self.cache.get(id).map(|ti| &ti.size)
	}

	/// Shows the texture on a specific position (i.e. makes it visible).
	///
	/// It will be visible after either `present` or `fade_in` is called.
	pub fn show(&mut self, id: Index, pos: Point) {
		self.screen.push(ScreenItem { id, pos });
	}

	/// Hides the texture from all visible positions.
	///
	/// Change will take affect after either `present` or `fade_in` is called.
	pub fn hide(&mut self, id: Index) {
		self.screen.retain(|i| i.id != id);
	}

	/// Removes the texture from the cache.
	///
	/// Change will take affect after either `present` or `fade_in` is called.
	pub fn remove(&mut self, id: Index) {
		self.hide(id);
		self.cache.remove(id);
	}

	pub fn remove_only(&mut self, ids: &[Index]) {
		ids.iter().for_each(|id| self.remove(*id));
	}

	/// Clears the texture cache.
	///
	/// Change will take affect after either `present` or `fade_in` is called.
	pub fn free(&mut self) {
		self.screen.clear();
		self.cache.clear();
	}

	/// Renders visible textures and updates the screen.
	pub fn present(&mut self) {
		for s in &self.screen {
			let t = self.cache.get(s.id).expect(&format!("[window.present] Element with id = {:?} is not found", s.id));

			self.canvas.copy(&t.txt, None, Rect::new(s.pos.x, s.pos.y, t.size.width, t.size.height)).unwrap();
		}

		self.canvas.present();
	}

	/// Fades in the screen.
	///
	/// NB: this is a blocking operation.
	pub fn fade_in(&mut self) {
		let mut instant = Instant::now();
		let rect = Rect::new(0, 0, 320, 200);

		// create a texture from all visible textures
		let mut txt = self.creator
			.create_texture_target(PIXEL_FORMAT_RGBA, rect.width(), rect.height())
			.unwrap();

		txt.set_blend_mode(BlendMode::Blend);

		let Self { ref screen, ref mut cache, .. } = self;

		self.canvas.with_texture_canvas(&mut txt, |tc| {
			for s in screen {
				let t = cache.get(s.id).expect(&format!("[window.fade_in] Element with id = {:?} is not found", s.id));

				tc.copy(&t.txt, None, Rect::new(s.pos.x, s.pos.y, t.size.width, t.size.height)).unwrap();
			}
		}).unwrap();

		for step in 0..128u8 {
			txt.set_alpha_mod(step * 2);

			self.canvas.clear();
			self.canvas.copy(&txt, None, rect).unwrap();
			self.canvas.present();

			let time_spent = instant.elapsed();
			thread::sleep(if FADE_STEP > time_spent { FADE_STEP - time_spent } else { FADE_STEP });
			instant = Instant::now();
		}
	}

	pub fn fade_in_only(&mut self, ids: &[Index]) {
		self.fade_only(ids, true);
	}

	/// Fades out the screen.
	///
	/// NB: this is a blocking operation.
	pub fn fade_out(&mut self) {
		let mut instant = Instant::now();
		let (width, height) = self.canvas.window().size();

		let prev_mode = self.canvas.blend_mode();
		self.canvas.set_blend_mode(BlendMode::Blend);

		for step in 0..128u8 {
			self.canvas.set_draw_color(Color::RGBA(0, 0, 0, step));
			self.canvas.fill_rect(Rect::new(0, 0, width, height)).unwrap();
			self.canvas.present();

			let time_spent = instant.elapsed();
			thread::sleep(if FADE_STEP > time_spent { FADE_STEP - time_spent } else { FADE_STEP });
			instant = Instant::now();
		}

		self.canvas.set_blend_mode(prev_mode);
	}

	pub fn fade_out_only(&mut self, ids: &[Index]) {
		self.fade_only(ids, false);
	}

	/// Fades out specific color from the palette.
	///
	/// NB: this is a blocking operation.
	pub fn fade_out_by_color_ix(&mut self, ix: usize) {
		let mut instant = Instant::now();

		let mut color = {
			match self.palette.get(ix * 3..ix * 3 + 3) {
				Some(rgb) => Color::RGBA(rgb[0] << 2, rgb[1] << 2, rgb[2] << 2, 255),
				None => return,
			}
		};

		let rect = Rect::new(0, 0, 320, 200);

		let mut txt = self.creator
			.create_texture_target(PIXEL_FORMAT_RGB, rect.width(), rect.height())
			.unwrap();

		let Self { ref screen, ref mut cache, .. } = self;
		let mut pixels = Vec::new();

		self.canvas.with_texture_canvas(&mut txt, |tc| {
			for s in screen {
				let t = cache.get(s.id).expect(&format!("[window.fade_out_by_color_ix] Element with id = {:?} is not found", s.id));

				tc.copy(&t.txt, None, Rect::new(s.pos.x, s.pos.y, t.size.width, t.size.height)).unwrap();
			}

			if let Ok(px_vec) = tc.read_pixels(rect, PIXEL_FORMAT_RGB) {
				pixels = px_vec;
			}
		}).unwrap();

		if pixels.is_empty() {
			return;
		}

		// store (x, y) for each pixel we're going to fade out
		let points: Vec<_> = pixels
			.chunks(3) // pixel data is a 3-byte (RGB) chunk
			.enumerate()
			.filter(|(_, x)| x[0..=2] == [color.r, color.g, color.b]) // skip alpha channel
			.map(|(i, _)| Point2::new(i as i32 % rect.width() as i32, i as i32 / rect.width() as i32))
			.collect();

		for _ in 1..=max(color.r, max(color.g, color.b)) {
			color.r -= min(color.r, 1);
			color.g -= min(color.g, 1);
			color.b -= min(color.b, 1);

			self.canvas.with_texture_canvas(&mut txt, |tc| {
				tc.set_draw_color(color);
				tc.draw_points(points.as_slice()).unwrap();
			}).unwrap();

			self.canvas.clear();
			self.canvas.copy(&txt, None, rect).unwrap();
			self.canvas.present();

			let time_spent = instant.elapsed();
			thread::sleep(if FADE_STEP > time_spent { FADE_STEP - time_spent } else { FADE_STEP });
			instant = Instant::now();
		}
	}

	/// Clears the screen (with the default color, which normally is black).
	///
	/// Change will take affect after `present` is called.
	pub fn clear(&mut self) {
		self.screen.clear();
		self.canvas.clear();
	}

	fn fade_only(&mut self, ids: &[Index], fade_in: bool) {
		let mut instant = Instant::now();
		let rect = Rect::new(0, 0, 320, 200);

		// create a background texture
		let mut back = self.creator
			.create_texture_target(PIXEL_FORMAT_RGBA, rect.width(), rect.height())
			.unwrap();

		let cache = &mut self.cache;
		let iter = self.screen.iter().filter(|s| !ids.contains(&s.id));

		self.canvas.with_texture_canvas(&mut back, |tc| {
			for s in iter {
				let t = cache.get(s.id).expect(&format!("[window.fade_only] Element with id = {:?} is not found", s.id));

				tc.copy(&t.txt, None, Rect::new(s.pos.x, s.pos.y, t.size.width, t.size.height)).unwrap();
			}
		}).unwrap();

		// create a foreground texture which is going to be faded
		let mut front = self.creator
			.create_texture_target(PIXEL_FORMAT_RGBA, rect.width(), rect.height())
			.unwrap();

		front.set_blend_mode(BlendMode::Blend);

		let iter = self.screen.iter().filter(|s| ids.contains(&s.id));

		self.canvas.with_texture_canvas(&mut front, |tc| {
			for s in iter {
				let t = cache.get(s.id).expect(&format!("[window.fade_only] Element with id = {:?} is not found", s.id));

				tc.copy(&t.txt, None, Rect::new(s.pos.x, s.pos.y, t.size.width, t.size.height)).unwrap();
			}
		}).unwrap();

		self.canvas.clear();

		if fade_in {
			for step in 0..128u8 {
				front.set_alpha_mod(step * 2);

				self.canvas.copy(&back, None, rect).unwrap();
				self.canvas.copy(&front, None, rect).unwrap();
				self.canvas.present();

				let time_spent = instant.elapsed();
				thread::sleep(if FADE_STEP > time_spent { FADE_STEP - time_spent } else { FADE_STEP });
				instant = Instant::now();
			}
		} else {
			for step in (0..128u8).rev() {
				front.set_alpha_mod(step * 2);

				self.canvas.copy(&back, None, rect).unwrap();
				self.canvas.copy(&front, None, rect).unwrap();
				self.canvas.present();

				let time_spent = instant.elapsed();
				thread::sleep(if FADE_STEP > time_spent { FADE_STEP - time_spent } else { FADE_STEP });
				instant = Instant::now();
			}
		}
	}
}

const SCREEN_SCALE: u8 = 2;

impl<'ctx> System<'ctx> for Window {
	type Dependencies = &'ctx WindowConfig;

	fn create(cfg: Self::Dependencies) -> Result<Self> {
		let context = sdl2::init().unwrap();
		let video = context.video().unwrap();

		let window = video
			.window(cfg.title, cfg.width as u32 * SCREEN_SCALE as u32, cfg.height as u32 * SCREEN_SCALE as u32)
			.position_centered()
			.build()?;

		let mut canvas = window.into_canvas().build().unwrap();
		canvas.set_scale(SCREEN_SCALE as f32, SCREEN_SCALE as f32).unwrap();

		let creator = canvas.texture_creator();

		Ok(Window {
			palette: Vec::new(),
			context,
			canvas,
			creator,
			cache: Arena::new(),
			screen: Vec::new(),
		})
	}
}
