use eyre::Result;
use winit::event::VirtualKeyCode;

use crate::{
	engine::State,
	game::options::Model,
	graphics::{Point, Size, Sprite, SCREEN_START},
	task::yield_now,
};

const ANIM_DELAY: u64 = 100;
const ANIM_SIZE: Size = Size::wh(88, 24); // 16 icons of 88x24 bytes each
const ANIM_POS: Point = Point::xy(91, 97);

const KEYS: &[(&str, &str)] = &[
	("I1E", "I1D"), // Esprit S4
	("I11", "I10"), // Elan SE
	("I13", "I12"), // M200
];

pub async fn select_model(state: &mut State) -> Result<Option<Model>> {
	let mut model = Model::default();

	let mut sprites: Option<(Sprite, Vec<Sprite>)> = None;
	let mut frame: Option<usize> = None;
	let mut fade_out = false;

	let selection = loop {
		yield_now().await;

		{
			let input = state.input.borrow();

			if input.key_pressed(VirtualKeyCode::Left) {
				model = model.prev();
				sprites = None;
				fade_out = true;
			} else if input.key_pressed(VirtualKeyCode::Right) {
				model = model.next();
				sprites = None;
				fade_out = true;
			} else if input.key_pressed(VirtualKeyCode::Return) {
				break Some(model);
			} else if input.key_pressed(VirtualKeyCode::Escape) {
				break None;
			}
		}

		if sprites.is_none() {
			let (bgr_key, ani_key) = KEYS[model as usize];
			let (bgr, pal) = state.arc.get_with_palette(bgr_key)?;
			let anim = state.arc.get(ani_key)?;

			state.screen.palette = pal;

			frame = None;
			sprites = Some((
				Sprite::from(bgr),
				anim.chunks((ANIM_SIZE.width * ANIM_SIZE.height) as usize)
					.map(|i| Sprite::from(i.to_vec()).with_size(ANIM_SIZE))
					.collect(),
			));
		}

		if fade_out {
			fade_out = false;

			state.screen.fade_out(None).await;
		}

		state.screen.draw(&sprites.as_ref().unwrap().0, SCREEN_START);

		frame = Some(match frame {
			Some(i) => {
				state.screen.draw(&sprites.as_ref().unwrap().1[i], ANIM_POS);
				state.screen.present();

				std::thread::sleep(std::time::Duration::from_millis(ANIM_DELAY));

				if i + 1 < sprites.as_ref().unwrap().1.len() {
					i + 1
				} else {
					0
				}
			}
			None => {
				state.screen.draw(&sprites.as_ref().unwrap().1[0], ANIM_POS);
				state.screen.fade_in(None).await;

				1
			}
		});
	};

	state.screen.fade_out(None).await;

	Ok(selection)
}
