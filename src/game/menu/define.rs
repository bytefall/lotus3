use eyre::Result;
use std::cell::Ref;
use winit::event::VirtualKeyCode;
use winput::WinitInputHelper;

use super::{FRAME_OFFSET, FRAME_SIZE_4R, FRAME_SIZE_ST};
use crate::{
	engine::State,
	graphics::{
		font::{Font, CHAR_SET_04},
		Drawable, Frame, Sprite, FRAME_BORDER, SCREEN_START,
	},
	task::yield_now,
};

#[derive(Default)]
struct Position {
	row: u8,
	editor: bool,
}

pub async fn define_menu(state: &mut State) -> Result<()> {
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
			state.screen.draw(&bgr, SCREEN_START);

			state.screen.print("XKXCJGFJH-33", &font, (117, 56).into());
			state.screen.print("         -00", &font, (117, 56 + 15 * 1).into());
			state.screen.print("         -00", &font, (117, 56 + 15 * 2).into());
			state.screen.print("         -00", &font, (117, 56 + 15 * 3).into());
			state.screen.print("         -00", &font, (117, 56 + 15 * 4).into());
			state.screen.print("         -00", &font, (117, 56 + 15 * 5).into());
			state.screen.print("         -00", &font, (117, 56 + 15 * 6).into());
			state.screen.print("         -00", &font, (117, 56 + 15 * 7).into());
			state.screen.print("         -00", &font, (117, 56 + 15 * 8).into());

			const COL: u8 = 1;

			let frame = Frame::new(if pos.row == 0 { FRAME_SIZE_ST } else { FRAME_SIZE_4R });

			state.screen.draw(
				&frame,
				(
					(COL as u32 * (frame.width() - FRAME_BORDER + 1) + FRAME_OFFSET.0),
					(pos.row as u32 * (FRAME_SIZE_ST.height - FRAME_BORDER + 1) + FRAME_OFFSET.1),
				)
					.into(),
			);

			if first_time {
				first_time = false;

				state.screen.fade_in(None).await;
			} else {
				state.screen.present();
			}
		}
	}

	state.screen.fade_out(None).await;

	Ok(())
}

fn handle_input(input: Ref<WinitInputHelper>, pos: &mut Position) -> (bool, bool) {
	let mut key_pressed = false;

	if !pos.editor {
		match pos.row {
			0 if input.key_pressed(VirtualKeyCode::Down) => {
				pos.row += 1;
				key_pressed = true;
			}
			1 if input.key_pressed(VirtualKeyCode::Up) => {
				pos.row -= 1;
				key_pressed = true;
			}
			_ => {}
		}
	}

	if input.key_pressed(VirtualKeyCode::Escape) {
		if pos.editor {
			pos.editor = false;
			key_pressed = true;
		} else {
			return (false, true);
		}
	}

	if input.key_pressed(VirtualKeyCode::Return) {
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
