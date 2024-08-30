use anyhow::Result;
use std::time::Instant;

use crate::{
	engine::State,
	graphics::{
		font::{Font, CHAR_SET_03},
		Size, Sprite,
	},
	input::{BACKSPACE_CHAR, ENTER_CHAR, ESCAPE_CHAR},
	screen::{fade_out, screen, screen_at},
	task::yield_now,
};

pub async fn protection(state: &mut State) -> Result<()> {
	const HELMET_SIZE: Size = Size::wh(48, 40); // 24 helmets of 48x40 bytes each

	let instant = Instant::now();

	let (i21, ref pal) = state.arc.get_with_palette("I21")?;
	let i22 = state.arc.get_series("I22", HELMET_SIZE.width * HELMET_SIZE.height)?;

	let bgr = Sprite::from(i21);
	let font = Font::from(CHAR_SET_03, state.arc.get("C03")?);
	let helmet1 = Sprite::from(i22[0].clone()).with_size(HELMET_SIZE);
	let helmet2 = Sprite::from(i22[1].clone()).with_size(HELMET_SIZE);

	let mut first_time = true;
	let mut enter_code: Option<String> = None;
	let mut code = String::new();

	'main: loop {
		yield_now().await;

		let mut key_pressed = false;

		for c in state.input.borrow().chars() {
			match c {
				ENTER_CHAR | ESCAPE_CHAR => {
					break 'main;
				}
				BACKSPACE_CHAR if code.pop().is_some() => {
					key_pressed = true;
				}
				_ if code.len() < 3 && (c.is_ascii_alphabetic() || c.is_ascii_digit()) => {
					code.push(c.to_ascii_uppercase());
					key_pressed = true;
				}
				_ => {}
			}
		}

		if first_time || key_pressed {
			first_time = false;

			bgr.draw(screen(), pal);
			helmet1.draw(screen_at((141, 13)), pal);
			helmet2.draw(screen_at((141, 73)), pal);

			font.print(
				screen_at((60, 140)),
				enter_code.get_or_insert_with(|| format!("ENTER CODE FOR WINDOW {}", instant.elapsed().as_millis())),
				pal,
			);
			font.print(screen_at((150, 165)), &code, pal);
		}
	}

	fade_out(None).await;

	Ok(())
}
