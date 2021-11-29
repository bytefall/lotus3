use eyre::Result;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::time::Instant;
use winit::{
	dpi::{LogicalPosition, PhysicalSize},
	event_loop::EventLoop,
	window::{Window as WinitWindow, WindowBuilder},
};

use crate::{
	ecs::system::System,
	graphics::{Canvas, Color, Drawable, Point, Printable, FadeType, SCREEN_BPP},
};

pub struct WindowConfig {
	pub title: &'static str,
	pub width: u32,
	pub height: u32,
}

pub struct Window {
	_window: WinitWindow,
	pub event_loop: Option<EventLoop<()>>,
	pixels: Pixels,
	pub palette: Vec<u8>,
}

impl Window {
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

	pub fn fade_in(&mut self) {
		let fade = |src, fade_factor| src * fade_factor;

		self.fade(fade, None, None);
	}

	pub fn fade_out(&mut self) {
		let fade = |src, fade_factor| src * (1.0 - fade_factor);

		self.fade(fade, None, None);
	}

	pub fn fade_out_by_color(&mut self, color: Color) {
		let fade = |src, fade_factor| src * (1.0 - fade_factor);

		const FADE_KEY: u8 = 100;

		let prepare = |src: &mut [u8]| {
			for px in src.chunks_exact_mut(SCREEN_BPP as usize)
				.filter(|x| x[0..=2] == [color.r, color.g, color.b]) {
					px[3] = FADE_KEY;
				}
		};

		let filter = |px: &[u8]| px[3] == FADE_KEY;

		self.fade(fade, Some(&prepare), Some(filter));
	}

	fn fade(&mut self, fade: fn(f64, f64) -> f64, prepare: Option<&dyn Fn(&mut [u8])>, filter: Option<fn(&[u8]) -> bool>) {
		let start = Instant::now();

		let mut source = self.pixels.get_frame().to_vec();

		if let Some (f) = prepare {
			f(&mut source);
		}

		loop {
			let ticks = Instant::now().duration_since(start).as_secs_f64() * 280.0;
			let fade_factor = (ticks / 6.0 / 16.0).clamp(0.0, 1.0);

			for (dst, src) in self.pixels.get_frame().chunks_exact_mut(SCREEN_BPP as usize)
				.zip(source.chunks_exact(SCREEN_BPP as usize))
				.filter(|(_, src)| filter.map_or(true, |f| f(src))) {
					dst[0] = fade(src[0] as f64, fade_factor).round() as u8;
					dst[1] = fade(src[1] as f64, fade_factor).round() as u8;
					dst[2] = fade(src[2] as f64, fade_factor).round() as u8;
				}

			self.pixels.render().unwrap();

			if fade_factor >= 1.0 {
				break;
			}
		}
	}

	pub fn fade_only(&mut self, fade: FadeType, back: &Canvas, front: &Canvas) {
		let start = Instant::now();

		loop {
			let ticks = Instant::now().duration_since(start).as_secs_f64() * 280.0;
			let fade_factor = (ticks / 6.0 / 16.0).clamp(0.0, 1.0);
			let alpha = if matches!(fade, FadeType::Out) { 1.0 - fade_factor } else { fade_factor };

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

			if fade_factor >= 1.0 {
				break;
			}
		}
	}

	pub fn clear(&mut self) {
		for pixel in self.pixels.get_frame().chunks_exact_mut(SCREEN_BPP as usize) {
			pixel[0] = 0;
			pixel[1] = 0;
			pixel[2] = 0;
			pixel[3] = 255;
		}
	}
}

const SCREEN_SCALE: u32 = 2;

impl<'ctx> System<'ctx> for Window {
	type Dependencies = &'ctx WindowConfig;

	fn create(cfg: Self::Dependencies) -> Result<Self> {
		let size = PhysicalSize::new(cfg.width * SCREEN_SCALE, cfg.height * SCREEN_SCALE);
		let event_loop = EventLoop::new();

		let window = WindowBuilder::new()
			.with_visible(false)
			.with_title(cfg.title)
			.with_inner_size(size)
			.with_min_inner_size(size)
			.with_resizable(false)
			.build(&event_loop)?;

		let size = window.inner_size();
		let pixels = PixelsBuilder::new(cfg.width, cfg.height, SurfaceTexture::new(size.width, size.height, &window))
			.device_descriptor(wgpu::DeviceDescriptor {
				label: None,
				features: wgpu::Features::empty(),
				limits: wgpu::Limits {
					max_dynamic_storage_buffers_per_pipeline_layout: 0,
					max_storage_buffers_per_shader_stage: 0,
					max_storage_textures_per_shader_stage: 0,
					max_storage_buffer_binding_size: 0,
					..wgpu::Limits::default()
				}
			})
			.wgpu_backend(wgpu::Backends::all())
			.build()?;

		let (monitor_width, monitor_height) = {
			if let Some(monitor) = window.current_monitor() {
				let size = monitor.size().to_logical(window.scale_factor());
				(size.width, size.height)
			} else {
				(cfg.width, cfg.height)
			}
		};

		let center = LogicalPosition::new(
			(monitor_width - cfg.width * SCREEN_SCALE) as f32 / 2.0,
			(monitor_height - cfg.height * SCREEN_SCALE) as f32 / 2.0,
		);

		window.set_outer_position(center);
		window.set_visible(true);

		Ok(Self {
			_window: window,
			event_loop: Some(event_loop),
			pixels,
			palette: Vec::new(),
		}).map(|mut win| { win.clear(); win })
	}
}
