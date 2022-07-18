use eyre::Result;
use winit::event::VirtualKeyCode;

use crate::{
	engine::State,
	graphics::{Sprite, SCREEN_START},
	task::yield_now,
};

pub async fn audio_tuner(state: &mut State) -> Result<Option<u8>> {
	let (bgr, pal) = state.arc.get_with_palette("I1C")?;
	state.screen.palette = pal;

	let bgr = Sprite::from(bgr);

	let mut first_time = true;
	let mut track_num = 1;

	let selection = loop {
		yield_now().await;

		{
			let input = state.input.borrow();

			if input.key_pressed(VirtualKeyCode::Left) {
				track_num -= 1;
			} else if input.key_pressed(VirtualKeyCode::Right) {
				track_num += 1;
			} else if input.key_pressed(VirtualKeyCode::Return) {
				break Some(track_num);
			} else if input.key_pressed(VirtualKeyCode::Escape) {
				break None;
			}
		}

		state.screen.draw(&bgr, SCREEN_START);
		// let font_c06 = Font::from(CHAR_SET_06, state.arc.get("C06")?);
		// state.screen.print(&font_c06, "01:02:03").show((60, 140).into());

		if first_time {
			first_time = false;

			state.screen.fade_in(None).await;
		} else {
			state.screen.present();
		}
	};

	state.screen.fade_out(None).await;

	Ok(selection)
}
