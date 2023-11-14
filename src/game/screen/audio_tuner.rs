use anyhow::Result;
use winit::keyboard::{Key, NamedKey};

use crate::{
	engine::State,
	graphics::Sprite,
	screen::{fade_in, fade_out, screen},
	task::yield_now,
};

pub async fn audio_tuner(state: &mut State) -> Result<Option<u8>> {
	let (bgr, ref pal) = state.arc.get_with_palette("I1C")?;

	let bgr = Sprite::from(bgr);

	let mut first_time = true;
	let mut track_num = 1;

	let selection = 'main: loop {
		yield_now().await;

		for k in state.input.borrow().keys() {
			match k {
				Key::Named(NamedKey::ArrowLeft) => track_num -= 1,
				Key::Named(NamedKey::ArrowRight) => track_num += 1,
				Key::Named(NamedKey::Enter) => break 'main Some(track_num),
				Key::Named(NamedKey::Escape) => break 'main None,
				_ => {}
			}
		}

		if first_time {
			first_time = false;

			bgr.draw(screen(), pal);
			// let font_c06 = Font::from(CHAR_SET_06, state.arc.get("C06")?);
			// state.screen.print(&font_c06, "01:02:03").show((60, 140).into());

			fade_in(None).await;
		}
	};

	fade_out(None).await;

	Ok(selection)
}
