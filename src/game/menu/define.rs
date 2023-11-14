use anyhow::Result;
use std::cell::Ref;
use winit::keyboard::NamedKey;

use super::{FRAME_OFFSET, FRAME_SIZE_4R, FRAME_SIZE_ST};
use crate::{
	engine::State,
	graphics::{
		font::{Font, CHAR_SET_04},
		Frame, Sprite, FRAME_BORDER,
	},
	input::InputHelper,
	screen::{fade_in, fade_out, screen, screen_at},
	task::yield_now,
};

#[derive(Default)]
struct Position {
	row: u8,
	editor: bool,
}

pub async fn define_menu(state: &mut State, pal: &[u8]) -> Result<()> {
	let bgr = Sprite::from(state.arc.get("I16")?);
	let font = Font::from(CHAR_SET_04, state.arc.get("C04")?);

	let mut first_time = true;
	let mut pos = Position::default();

	loop {
		yield_now().await;

		let (key_pressed, exit) = handle_input(state.input.borrow(), &mut pos);

		if exit {
			break;
		}

		if first_time || key_pressed {
			bgr.draw(screen(), pal);

			font.print(screen_at((117, 56)), "XKXCJGFJH-33", pal);
			font.print(screen_at((117, 56 + 15)), "         -00", pal);
			font.print(screen_at((117, 56 + 15 * 2)), "         -00", pal);
			font.print(screen_at((117, 56 + 15 * 3)), "         -00", pal);
			font.print(screen_at((117, 56 + 15 * 4)), "         -00", pal);
			font.print(screen_at((117, 56 + 15 * 5)), "         -00", pal);
			font.print(screen_at((117, 56 + 15 * 6)), "         -00", pal);
			font.print(screen_at((117, 56 + 15 * 7)), "         -00", pal);
			font.print(screen_at((117, 56 + 15 * 8)), "         -00", pal);

			const COL: u8 = 1;

			let frame = Frame::new(if pos.row == 0 { FRAME_SIZE_ST } else { FRAME_SIZE_4R });
			frame.draw(
				screen_at((
					(COL as u32 * (frame.size.width - FRAME_BORDER + 1) + FRAME_OFFSET.0),
					(pos.row as u32 * (FRAME_SIZE_ST.height - FRAME_BORDER + 1) + FRAME_OFFSET.1),
				)),
				pal,
			);

			if first_time {
				first_time = false;

				fade_in(None).await;
			}
		}
	}

	fade_out(None).await;

	Ok(())
}

fn handle_input(input: Ref<InputHelper>, pos: &mut Position) -> (bool, bool) {
	let mut key_pressed = false;

	if !pos.editor {
		match pos.row {
			0 if input.key_pressed(NamedKey::ArrowDown) => {
				pos.row += 1;
				key_pressed = true;
			}
			1 if input.key_pressed(NamedKey::ArrowUp) => {
				pos.row -= 1;
				key_pressed = true;
			}
			_ => {}
		}
	}

	if input.key_pressed(NamedKey::Escape) {
		if pos.editor {
			pos.editor = false;
			key_pressed = true;
		} else {
			return (false, true);
		}
	}

	if input.key_pressed(NamedKey::Enter) {
		match pos.row {
			0 => return (false, true),
			1 => {
				pos.editor = !pos.editor;
				key_pressed = true;
			}
			_ => {}
		}
	}

	(key_pressed, false)
}
