use anyhow::Result;
use std::time::Instant;
use winit::event::VirtualKeyCode;
use winput::TextChar;

use crate::{
	engine::State,
	graphics::{
		font::{Font, CHAR_SET_03},
		Size, Sprite, SCREEN_START,
	},
	task::yield_now,
};

pub async fn protection(state: &mut State) -> Result<()> {
	const HELMET_SIZE: Size = Size::wh(48, 40); // 24 helmets of 48x40 bytes each

	let instant = Instant::now();

	let i22 = state.arc.get("I22")?;
	let mut i22 = i22.chunks((HELMET_SIZE.width * HELMET_SIZE.height) as usize);

	let (i21, pal) = state.arc.get_with_palette("I21")?;
	state.screen.palette = pal;

	let bgr = Sprite::from(i21);
	let font = Font::from(CHAR_SET_03, state.arc.get("C03")?);
	let helmet1 = Sprite::from(i22.next().unwrap().to_vec()).with_size(HELMET_SIZE);
	let helmet2 = Sprite::from(i22.next().unwrap().to_vec()).with_size(HELMET_SIZE);

	let mut first_time = true;
	let mut enter_code: Option<String> = None;
	let mut code = String::new();

	loop {
		yield_now().await;

		let input = state.input.borrow();

		if input.key_pressed(VirtualKeyCode::Return) || input.key_pressed(VirtualKeyCode::Escape) {
			break;
		}

		let mut key_pressed = false;

		for key in &input.text() {
			match key {
				TextChar::Char(c) if code.len() < 3 => {
					if c.is_ascii_alphabetic() || c.is_ascii_digit() {
						code.push(c.to_ascii_uppercase());

						key_pressed = true;
					}
				}
				TextChar::Back if code.pop().is_some() => {
					key_pressed = true;
				}
				_ => {}
			}
		}

		if first_time || key_pressed {
			first_time = false;

			state.screen.draw(&bgr, SCREEN_START);
			state.screen.draw(&helmet1, (141, 13).into());
			state.screen.draw(&helmet2, (141, 73).into());
			state.screen.print(
				enter_code.get_or_insert_with(|| format!("ENTER CODE FOR WINDOW {}", instant.elapsed().as_millis())),
				&font,
				(60, 140).into(),
			);
			state.screen.print(&code, &font, (150, 165).into());
			state.screen.present();
		}
	}

	state.screen.fade_out(None).await;

	Ok(())
}
