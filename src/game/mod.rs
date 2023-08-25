use anyhow::Result;

use crate::engine::State;

mod intro;
mod menu;
mod screen;

pub mod options;

use intro::*;
use menu::*;
use screen::*;

pub async fn main(mut state: State) -> Result<()> {
	protection(&mut state).await?;

	let mut ok = true; // ok = false when an async operation has been cancelled
	ok = ok && show_gremlin(&mut state).await?;
	ok = ok && show_magnetic_fields(&mut state).await?;
	ok = ok && show_credits(&mut state).await?;
	ok = ok && show_lotus_logo(&mut state).await?;
	ok = ok && show_magazine(&mut state).await?;

	if !ok {
		state.screen.fade_out(None).await;
	}

	let mut play_demo = true;

	loop {
		if play_demo {
			/*loop { // for each demo
				ok = show_lotus_logo(&mut state).await?;
				// DEMO
				// SCORE

				if !ok {
					state.screen.fade_out(None).await;
					break;
				}
			}*/
		}

		play_demo = false;

		match main_menu(&mut state).await? {
			Action::Start => (),
			Action::Exit => break,
		}

		let _model = match select_model(&mut state).await? {
			Some(model) => model,
			None => continue,
		};

		// if SOUND != OFF:
		let _track = match audio_tuner(&mut state).await? {
			Some(track) => track,
			None => continue,
		};

		// play_demo = true;

		break;
	}

	Ok(())
}
