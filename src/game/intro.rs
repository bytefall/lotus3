use eyre::Result;

use crate::{
	engine::State,
	graphics::{Canvas, Color, FadeType, Point, Size, Sprite, SpriteFont, SCREEN_START},
	task::sleep,
};

macro_rules! return_false {
	($token:expr) => {
		if $token.borrow().cancelled() {
			$token.borrow_mut().clear();

			return Ok(false);
		}
	};
}

macro_rules! try_sleep {
	($task:expr, $token:expr) => {
		$task.with_token($token).await;

		return_false!($token);
	};
}

macro_rules! try_fade_in {
	($screen:expr, $token:expr) => {
		$screen.fade_in(Some($token)).await;

		return_false!($token);
	};
}

macro_rules! try_fade_out {
	($screen:expr, $token:expr) => {
		$screen.fade_out(Some($token)).await;

		return_false!($token);
	};
}

macro_rules! try_fade_out_by_color {
	($screen:expr, $color:expr, $token:expr) => {
		$screen.fade_out_by_color($color, Some($token)).await;

		return_false!($token);
	};
}

macro_rules! try_fade_only {
	($screen:expr, $fade_type:expr, $back:expr, $front:expr, $token:expr) => {
		$screen.fade_only($fade_type, $back, $front, Some($token)).await;

		return_false!($token);
	};
}

pub async fn show_gremlin(state: &mut State) -> Result<bool> {
	const SPLASH_SIZE: Size = Size::wh(16, 8);

	let (q00, pal) = state.arc.get_with_palette("Q00")?;

	state.screen.palette = pal;
	state.screen.draw(&Sprite::from(q00), SCREEN_START);

	try_fade_in!(state.screen, &state.token);
	try_sleep!(sleep(200), &state.token);

	let q01 = state.arc.get_series("Q01", SPLASH_SIZE.width * SPLASH_SIZE.height)?;

	for i in [0usize, 1, 2, 3, 2, 1, 0] {
		let timer = sleep(100);

		state.screen.draw(
			&Sprite::from(q01.get(i).unwrap().to_vec()).with_size(SPLASH_SIZE),
			(112, 85).into(),
		);
		state.screen.present();

		try_sleep!(timer, &state.token);
	}

	for i in [4usize, 5, 6, 7, 6, 5, 4] {
		let timer = sleep(100);

		state.screen.draw(
			&Sprite::from(q01.get(i).unwrap().to_vec()).with_size(SPLASH_SIZE),
			(144, 110).into(),
		);
		state.screen.present();

		try_sleep!(timer, &state.token);
	}

	try_fade_out!(state.screen, &state.token);

	Ok(true)
}

pub async fn show_magnetic_fields(state: &mut State) -> Result<bool> {
	const KEYS: [&str; 22] = [
		"Q02", "Q03", "Q04", "Q05", "Q06", "Q07", "Q08", "Q09", "Q0A", "Q0B", "Q0C", "Q0D", "Q0E", "Q0F", "Q10", "Q11",
		"Q12", "Q13", "Q14", "Q15", "Q16", "Q17",
	];

	let (_, pal) = state.arc.get_with_palette(KEYS.last().unwrap())?;

	state.screen.palette = pal;

	for key in KEYS {
		let timer = sleep(50);

		state.screen.draw(&Sprite::from(state.arc.get(key)?), SCREEN_START);
		state.screen.present();

		try_sleep!(timer, &state.token);
	}

	try_sleep!(sleep(1000), &state.token);
	try_fade_out!(state.screen, &state.token);

	Ok(true)
}

pub async fn show_credits(state: &mut State) -> Result<bool> {
	const CREDITS_FADE_IN_TIMEOUT: u64 = 2000;
	const CREDITS_FADE_OUT_TIMEOUT: u64 = 1000;
	const CREDITS: [Option<(&str, u32, u32)>; 26] = [
		Some(("A GAME", 118, 43)),
		Some(("BY", 146, 67)),
		Some(("ANDREW MORRIS", 69, 91)),
		Some(("AND", 139, 115)),
		Some(("SHAUN SOUTHERN", 62, 139)),
		None,
		Some(("LEVEL DESIGN", 76, 67)),
		Some(("BY", 146, 91)),
		Some(("PETER LIGGETT", 69, 115)),
		None,
		Some(("MUSIC", 125, 67)),
		Some(("BY", 146, 91)),
		Some(("PATRICK PHELAN", 62, 115)),
		None,
		Some(("PC CONVERSION", 69, 43)),
		Some(("BY", 146, 67)),
		Some(("JON MEDHURST FOR", 48, 91)),
		Some(("CYGNUS SOFTWARE", 55, 115)),
		Some(("ENGINEERING LTD.", 52, 139)),
		None,
		Some(("COPYRIGHT 1993", 62, 43)),
		Some(("MAGNETIC FIELDS", 55, 67)),
		Some(("(SOFTWARE DESIGN) LTD.", 10, 91)),
		Some(("GREMLIN GRAPHICS", 48, 115)),
		Some(("SOFTWARE LTD.", 73, 139)),
		None,
	];

	let (q19, pal) = state.arc.get_with_palette("Q19")?;

	let bgr = Sprite::from(q19);
	state.screen.palette = pal;
	state.screen.draw(&bgr, SCREEN_START);

	try_fade_in!(state.screen, &state.token);
	try_sleep!(sleep(2000), &state.token);

	let mut front = Canvas::default();
	let mut back = Canvas::default();
	back.draw(&bgr, &state.screen.palette, SCREEN_START);

	let font = SpriteFont::from(state.arc.get("Q1A")?);

	for item in &CREDITS {
		match item {
			Some(i) => {
				front.print(i.0, &font, &state.screen.palette, (i.1, i.2).into());
			}
			None => {
				try_fade_only!(state.screen, FadeType::In, &back, &front, &state.token);
				try_sleep!(sleep(CREDITS_FADE_IN_TIMEOUT), &state.token);

				try_fade_only!(state.screen, FadeType::Out, &back, &front, &state.token);
				try_sleep!(sleep(CREDITS_FADE_OUT_TIMEOUT), &state.token);

				front = Canvas::default();
			}
		}
	}

	let q1b = state.arc.get("Q1B")?;

	for step in 1..=36 {
		let timer = sleep(50);

		let mut front = Canvas::default();
		draw_a_car(&mut front, &q1b, &state.screen.palette, step);

		state.screen.draw(&bgr, SCREEN_START);
		state.screen.blit(&front);
		state.screen.present();

		try_sleep!(timer, &state.token);
	}

	for key in ["Q1C", "Q1D"] {
		let timer = sleep(50);

		state.screen.draw(&Sprite::from(state.arc.get(key)?), SCREEN_START);
		state.screen.present();

		try_sleep!(timer, &state.token);
	}

	let q1e = state.arc.get("Q1E")?;
	let color_ix = q1e[0] as usize;

	state.screen.draw(&Sprite::from(q1e), SCREEN_START);
	state.screen.present();

	try_sleep!(sleep(2000), &state.token);

	let color = state
		.screen
		.palette
		.get(color_ix * 3..color_ix * 3 + 3)
		.map(|rgb| Color::rgb(rgb[0] << 2, rgb[1] << 2, rgb[2] << 2));

	try_fade_out_by_color!(state.screen, color.unwrap(), &state.token);
	try_sleep!(sleep(4000), &state.token);
	try_fade_out!(state.screen, &state.token);

	Ok(true)
}

fn draw_a_car(canvas: &mut Canvas, data: &[u8], pal: &[u8], step: usize) {
	const WIDTH: usize = 336; // NB: not 320!

	let cx = 256 + (36 - step) * 512;
	let di = 160 - ((((WIDTH * 170) + 224) / cx) >> 1) + (WIDTH * 64)
		- (((((WIDTH * 118) + 288) / cx) * (WIDTH * 32 + 170)) >> 16) * WIDTH;

	let xx = di % WIDTH;
	let mut y = di / WIDTH;

	for row in (0..WIDTH * 118).step_by(cx) {
		let offset = (row >> 8) * 224;
		let mut x = xx;

		for i in (offset..offset + 224).step_by(cx >> 8) {
			let px = data[i] as usize;

			if px != 0xFF {
				canvas.point(
					(pal[px * 3 + 0], pal[px * 3 + 1], pal[px * 3 + 2]).into(),
					(x as u32, y as u32).into(),
				);
			}

			x += 1;
		}

		y += 1;
	}
}

pub async fn show_lotus_logo(state: &mut State) -> Result<bool> {
	let (q18, pal) = state.arc.get_with_palette("Q18")?;

	state.screen.palette = pal;
	state.screen.draw(&Sprite::from(q18), SCREEN_START);

	try_fade_in!(state.screen, &state.token);
	try_sleep!(sleep(2000), &state.token);
	try_fade_out!(state.screen, &state.token);

	Ok(true)
}

pub async fn show_magazine(state: &mut State) -> Result<bool> {
	const VIDEO_SIZE: Size = Size::wh(160, 112);
	const VIDEO_POS: Point = Point::xy(136, 38);

	let (v32, pal) = state.arc.get_with_palette("V32")?;

	state.screen.palette = pal;

	fn get_with_leading_pal(mut raw: Vec<u8>, pal: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
		let dat = raw.split_off(720);

		let mut pal = pal.to_vec();
		let _ = pal.splice(16 * 3.., raw);

		Ok((dat, pal))
	}

	const KEYS: [&str; 41] = [
		"V00", "V01", "V02", "V03", "V04", "V05", "V06", "V07", "V08", "V09", "V0A", "V0B", "V0C", "V0D", "V0E", "V0F",
		"V10", "V11", "V12", "V13", "V14", "V15", "V16", "V17", "V18", "V19", "V1A", "V1B", "V1C", "V1D", "V1E", "V1F",
		"V20", "V21", "V22", "V23", "V24", "V25", "V26", "V27", "V28",
	];

	let mut back = Canvas::default();
	let mut front = Canvas::default();

	let bgr = Sprite::from(v32);
	back.draw(&bgr, &state.screen.palette, SCREEN_START);
	state.screen.draw(&bgr, SCREEN_START);

	for key in KEYS {
		let timer = sleep(100);

		let (vxx, pal) = get_with_leading_pal(state.arc.get(key)?, &state.screen.palette)?;
		let vxx = Sprite::from(vxx).with_size(VIDEO_SIZE);

		state.screen.palette = pal;
		state.screen.draw(&vxx, VIDEO_POS);

		if Some(&key) == KEYS.first() {
			try_fade_in!(state.screen, &state.token);
		} else {
			state.screen.present();

			// save the last frame
			if Some(&key) == KEYS.last() {
				front.draw(&vxx, &state.screen.palette, VIDEO_POS);
			}
		}

		try_sleep!(timer, &state.token);
	}

	try_fade_only!(state.screen, FadeType::Out, &back, &front, &state.token);

	let (v33, pal) = get_with_leading_pal(state.arc.get("V33")?, &state.screen.palette)?;

	state.screen.palette = pal;
	front.draw(
		&Sprite::from(v33).with_size(VIDEO_SIZE),
		&state.screen.palette,
		VIDEO_POS,
	);

	try_fade_only!(state.screen, FadeType::In, &back, &front, &state.token);
	try_sleep!(sleep(2000), &state.token);
	try_fade_out!(state.screen, &state.token);

	Ok(true)
}
