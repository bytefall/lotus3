use std::time::Instant;

use crate::{
	graphics::{Canvas, Color, Point},
	task::yield_now,
};

static mut SCREEN_BUFFER: Canvas = Canvas::new();
static mut IS_DIRTY: bool = false;

pub type CancelFn<'a> = Option<&'a dyn Fn() -> bool>;

pub fn screen() -> &'static mut [u32] {
	unsafe {
		IS_DIRTY = true;
		SCREEN_BUFFER.raw()
	}
}

pub fn screen_at(pos: impl Into<Point>) -> &'static mut [u32] {
	unsafe {
		IS_DIRTY = true;
		SCREEN_BUFFER.raw_at(pos)
	}
}

pub fn screen_copy() -> Canvas {
	unsafe { SCREEN_BUFFER }
}

pub fn get_screen_state() -> bool {
	unsafe { IS_DIRTY }
}

pub fn set_screen_state() {
	unsafe {
		IS_DIRTY = false;
	}
}

pub async fn fade_in(cancel: CancelFn<'_>) -> bool {
	let fade = |src, factor| src * factor;

	fade_impl(fade, None, cancel).await
}

pub async fn fade_out(cancel: CancelFn<'_>) -> bool {
	let fade = |src, factor| src * (1.0 - factor);

	fade_impl(fade, None, cancel).await
}

pub async fn fade_out_by_color(color: Color, cancel: CancelFn<'_>) -> bool {
	let fade = |src, factor| src * (1.0 - factor);
	let filter = move |c: u32| c == u32::from_be_bytes([255, color.r, color.g, color.b]);

	fade_impl(fade, Some(Box::new(filter)), cancel).await
}

async fn fade_impl(fade: fn(f64, f64) -> f64, filter: Option<Box<dyn Fn(u32) -> bool>>, cancel: CancelFn<'_>) -> bool {
	let start = Instant::now();

	// let src = screen_copy().0;
	let src = screen().to_vec();

	loop {
		let ticks = Instant::now().duration_since(start).as_secs_f64() * 280.0;
		let factor = (ticks / 6.0 / 16.0).clamp(0.0, 1.0);

		for (dst, src) in screen()
			.iter_mut()
			.zip(src.iter().copied())
			.filter(|(_, src)| filter.as_ref().map_or(true, |f| f(*src)))
		{
			let [_, r, g, b] = src.to_be_bytes();

			*dst = u32::from_be_bytes([
				255,
				fade(r as f64, factor).round() as u8,
				fade(g as f64, factor).round() as u8,
				fade(b as f64, factor).round() as u8,
			]);
		}

		yield_now().await;

		if factor >= 1.0 {
			break;
		}

		if cancel.as_ref().map_or(false, |f| f()) {
			return true;
		}
	}

	false
}

pub async fn fade_in_only(back: &Canvas, front: &Canvas, cancel: CancelFn<'_>) -> bool {
	let fade = |factor| factor;

	fade_only(fade, back, front, cancel).await
}

pub async fn fade_out_only(back: &Canvas, front: &Canvas, cancel: CancelFn<'_>) -> bool {
	let fade = |factor| 1.0 - factor;

	fade_only(fade, back, front, cancel).await
}

async fn fade_only(fade: fn(f64) -> f64, back: &Canvas, front: &Canvas, cancel: CancelFn<'_>) -> bool {
	fn blend_color(src: u8, lay: u8, alpha: f64) -> u8 {
		let src = src as f64;
		let lay = lay as f64;

		((1.0 - alpha) * src + alpha * lay).round() as u8
	}

	let start = Instant::now();

	loop {
		let ticks = Instant::now().duration_since(start).as_secs_f64() * 280.0;
		let factor = (ticks / 6.0 / 16.0).clamp(0.0, 1.0);
		let alpha = fade(factor);

		let mut dst_iter = screen().iter_mut();
		let mut src_iter = back.get().iter();
		let mut lay_iter = front.get().iter();

		while let (Some(dst), Some(src), Some(lay)) = (dst_iter.next(), src_iter.next(), lay_iter.next()) {
			let lay = lay.to_be_bytes();

			if lay[0] != 0 {
				let src = src.to_be_bytes();

				*dst = u32::from_be_bytes([
					255,
					blend_color(src[1], lay[1], alpha),
					blend_color(src[2], lay[2], alpha),
					blend_color(src[3], lay[3], alpha),
				]);
			}
		}

		yield_now().await;

		if factor >= 1.0 {
			break;
		}

		if cancel.as_ref().map_or(false, |f| f()) {
			return true;
		}
	}

	false
}
