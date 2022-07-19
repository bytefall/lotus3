use eyre::Result;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::time::Instant;
use winit::window::Window;

use crate::{
	graphics::{Canvas, Color, Drawable, FadeType, Point, Printable, SCREEN_BPP, SCREEN_HEIGHT, SCREEN_WIDTH},
	task::yield_now,
};

pub struct Screen {
	pixels: Pixels,
	pub palette: Vec<u8>,
}

impl Screen {
	pub fn from_window(window: &Window) -> Result<Self> {
		let size = window.inner_size();

		let pixels = PixelsBuilder::new(
			SCREEN_WIDTH,
			SCREEN_HEIGHT,
			SurfaceTexture::new(size.width, size.height, window),
		)
		.device_descriptor(wgpu::DeviceDescriptor {
			label: None,
			features: wgpu::Features::empty(),
			limits: wgpu::Limits {
				max_dynamic_storage_buffers_per_pipeline_layout: 0,
				max_storage_buffers_per_shader_stage: 0,
				max_storage_textures_per_shader_stage: 0,
				max_storage_buffer_binding_size: 0,
				..wgpu::Limits::default()
			},
		})
		.wgpu_backend(wgpu::Backends::all())
		.build()?;

		Ok(Self {
			pixels,
			palette: Vec::new(),
		})
	}

	pub fn draw(&mut self, sprite: &dyn Drawable, pos: Point) {
		let buffer = &mut self.pixels.get_frame()[pos.range()];

		sprite.draw(buffer, &self.palette);
	}

	pub fn print(&mut self, text: &str, font: &dyn Printable, pos: Point) {
		let buffer = &mut self.pixels.get_frame()[pos.range()];

		font.print(buffer, &self.palette, text);
	}

	pub fn blit(&mut self, canvas: &Canvas) {
		let mut dst_iter = self.pixels.get_frame().chunks_exact_mut(SCREEN_BPP as usize);
		let mut src_iter = canvas.get().chunks_exact(SCREEN_BPP as usize);

		while let (Some(dst), Some(src)) = (dst_iter.next(), src_iter.next()) {
			if src[3] != 0 {
				dst[0] = src[0];
				dst[1] = src[1];
				dst[2] = src[2];
				dst[3] = src[3];
			}
		}
	}

	pub fn present(&mut self) {
		self.pixels.render().unwrap();
	}

	pub async fn fade_in(&mut self, cancel: Option<&dyn Fn() -> bool>) -> bool {
		let fade = |src, factor| src * factor;

		self.fade(fade, None, None, cancel).await
	}

	pub async fn fade_out(&mut self, cancel: Option<&dyn Fn() -> bool>) -> bool {
		let fade = |src, factor| src * (1.0 - factor);

		self.fade(fade, None, None, cancel).await
	}

	pub async fn fade_out_by_color(&mut self, color: Color, cancel: Option<&dyn Fn() -> bool>) -> bool {
		let fade = |src, factor| src * (1.0 - factor);

		const FADE_KEY: u8 = 100;

		let prepare = |src: &mut [u8]| {
			for px in src
				.chunks_exact_mut(SCREEN_BPP as usize)
				.filter(|x| x[0..=2] == [color.r, color.g, color.b])
			{
				px[3] = FADE_KEY;
			}
		};

		let filter = |px: &[u8]| px[3] == FADE_KEY;

		self.fade(fade, Some(&prepare), Some(filter), cancel).await
	}

	async fn fade(
		&mut self,
		fade: fn(f64, f64) -> f64,
		prepare: Option<&dyn Fn(&mut [u8])>,
		filter: Option<fn(&[u8]) -> bool>,
		cancel: Option<&dyn Fn() -> bool>,
	) -> bool {
		let start = Instant::now();
		let mut source = self.pixels.get_frame().to_vec();

		if let Some(f) = prepare {
			f(&mut source);
		}

		loop {
			let ticks = Instant::now().duration_since(start).as_secs_f64() * 280.0;
			let factor = (ticks / 6.0 / 16.0).clamp(0.0, 1.0);

			for (dst, src) in self
				.pixels
				.get_frame()
				.chunks_exact_mut(SCREEN_BPP as usize)
				.zip(source.chunks_exact(SCREEN_BPP as usize))
				.filter(|(_, src)| filter.map_or(true, |f| f(src)))
			{
				dst[0] = fade(src[0] as f64, factor).round() as u8;
				dst[1] = fade(src[1] as f64, factor).round() as u8;
				dst[2] = fade(src[2] as f64, factor).round() as u8;
			}

			self.pixels.render().unwrap();

			if factor >= 1.0 {
				break;
			}

			yield_now().await;

			if cancel.as_ref().map_or(false, |f| f()) {
				return true;
			}
		}

		false
	}

	pub async fn fade_only(
		&mut self,
		fade: FadeType,
		back: &Canvas,
		front: &Canvas,
		cancel: Option<&dyn Fn() -> bool>,
	) -> bool {
		let start = Instant::now();

		loop {
			let ticks = Instant::now().duration_since(start).as_secs_f64() * 280.0;
			let factor = (ticks / 6.0 / 16.0).clamp(0.0, 1.0);
			let alpha = if matches!(fade, FadeType::Out) {
				1.0 - factor
			} else {
				factor
			};

			let mut dst_iter = self.pixels.get_frame().chunks_exact_mut(SCREEN_BPP as usize);
			let mut src_iter = back.get().chunks_exact(SCREEN_BPP as usize);
			let mut lay_iter = front.get().chunks_exact(SCREEN_BPP as usize);

			fn blend_color(src: u8, lay: u8, alpha: f64) -> u8 {
				let src = src as f64;
				let lay = lay as f64;

				((1.0 - alpha) * src + alpha * lay).round() as u8
			}

			while let (Some(dst), Some(src), Some(lay)) = (dst_iter.next(), src_iter.next(), lay_iter.next()) {
				if lay[3] != 0 {
					dst[0] = blend_color(src[0], lay[0], alpha);
					dst[1] = blend_color(src[1], lay[1], alpha);
					dst[2] = blend_color(src[2], lay[2], alpha);
				}
			}

			self.pixels.render().unwrap();

			if factor >= 1.0 {
				break;
			}

			yield_now().await;

			if cancel.as_ref().map_or(false, |f| f()) {
				return true;
			}
		}

		false
	}

	/*pub fn clear(&mut self) {
		for pixel in self.pixels.get_frame().chunks_exact_mut(SCREEN_BPP as usize) {
			pixel[0] = 0;
			pixel[1] = 0;
			pixel[2] = 0;
			pixel[3] = 255;
		}
	}*/
}
