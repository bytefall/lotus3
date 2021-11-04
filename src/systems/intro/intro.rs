use eyre::Result;

use crate::{
	data::Archive,
	ecs::system::System,
	game::{
		script::{ALL, BACK, FRONT, CommandSequence},
		state::{GameState, GameFlow},
	},
	graphics::{Point, PaintCanvas, PaintFn, Size, Sprite, SCREEN_START, WIDTH, HEIGHT},
	systems::{Input, Window},
};

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		arc: &'ctx Archive,
		cmd: &'ctx mut CommandSequence,
		input: &'ctx mut Input,
		flow: &'ctx mut GameFlow,
		win: &'ctx mut Window,
	}
}

pub struct Intro;

impl<'ctx> System<'ctx> for Intro {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Result<Self> {
		Ok(Self)
	}

	fn update(&mut self, mut dep: Self::Dependencies) -> Result<()> {
		if dep.flow.current() != &GameState::Intro {
			return Ok(());
		}

		if !dep.input.keys.is_empty() {
			dep.cmd.clear();
			dep.input.keys.clear();
			dep.win.fade_out();
			dep.win.free();
			dep.flow.set(GameState::main_menu());

			return Ok(());
		}

		if dep.flow.changed {
			dep.cmd.clear();
			dep.flow.changed = false;

			show_gremlin(&mut dep)?;
			show_magnetic_fields(&mut dep)?;
			show_credits(&mut dep)?;
			show_lotus_logo(&mut dep)?;
			show_magazine(&mut dep)?;
		}

		Ok(())
	}
}

fn show_gremlin(dep: &mut Dependencies) -> Result<()> {
	const SPLASH_SIZE: Size = Size::wh(16, 8);

	let (q00, pal) = dep.arc.get_with_palette("Q00")?;

	dep.cmd.batch(Some(200))
		.palette(pal)
		.draw(BACK, Sprite::from(q00), SCREEN_START)
		.fade_in(ALL);

	let q01 = dep.arc.get_series("Q01", SPLASH_SIZE.width * SPLASH_SIZE.height)?;

	for i in &[0usize, 1, 2, 3, 2, 1, 0] {
		dep.cmd.batch(Some(100))
			.clear(FRONT)
			.draw(FRONT, Sprite::from(q01.get(*i).unwrap().to_vec()).with_size(SPLASH_SIZE), Point::xy(112, 85))
			.present();
	}

	for i in &[4usize, 5, 6, 7, 6, 5, 4] {
		dep.cmd.batch(Some(100))
			.clear(FRONT)
			.draw(FRONT, Sprite::from(q01.get(*i).unwrap().to_vec()).with_size(SPLASH_SIZE), Point::xy(144, 110))
			.present();
	}

	dep.cmd.batch(None)
		.fade_out(ALL);

	Ok(())
}

fn show_magnetic_fields(dep: &mut Dependencies) -> Result<()> {
	const KEYS: [&str; 22] = [
		"Q02", "Q03", "Q04", "Q05", "Q06", "Q07", "Q08", "Q09", "Q0A", "Q0B", "Q0C", "Q0D",
		"Q0E", "Q0F", "Q10", "Q11", "Q12", "Q13", "Q14", "Q15", "Q16", "Q17",
	];

	let (_, pal) = dep.arc.get_with_palette(KEYS.last().unwrap())?;

	dep.cmd.batch(None)
		.clear(ALL)
		.palette(pal);

	for key in &KEYS {
		dep.cmd.batch(Some(50))
			.clear(BACK)
			.draw(BACK, Sprite::from(dep.arc.get(key)?), SCREEN_START)
			.present();
	}

	dep.cmd.batch(Some(1000));

	dep.cmd.batch(None)
		.fade_out(ALL);

	Ok(())
}

fn show_credits(dep: &mut Dependencies) -> Result<()> {
	const CREDITS_FADE_IN_TIMEOUT: Option<u16> = Some(2000);
	const CREDITS_FADE_OUT_TIMEOUT: Option<u16> = Some(1000);
	const CAR_SIZE: Size = Size::wh(WIDTH as u32, HEIGHT as u32);

	let (q19, pal) = dep.arc.get_with_palette("Q19")?;

	dep.cmd.batch(Some(2000))
		.clear(ALL)
		.palette(pal)
		.draw(BACK, Sprite::from(q19), SCREEN_START)
		.fade_in(ALL);

	dep.cmd.batch(CREDITS_FADE_IN_TIMEOUT)
		.clear(FRONT)
		.print(FRONT, "A GAME", Point::xy(118, 43))
		.print(FRONT, "BY", Point::xy(146, 67))
		.print(FRONT, "ANDREW MORRIS", Point::xy(69, 91))
		.print(FRONT, "AND", Point::xy(139, 115))
		.print(FRONT, "SHAUN SOUTHERN", Point::xy(62, 139))
		.fade_in(FRONT);

	dep.cmd.batch(CREDITS_FADE_OUT_TIMEOUT)
		.fade_out(FRONT);

	dep.cmd.batch(CREDITS_FADE_IN_TIMEOUT)
		.print(FRONT, "LEVEL DESIGN", Point::xy(76, 67))
		.print(FRONT, "BY", Point::xy(146, 91))
		.print(FRONT, "PETER LIGGETT", Point::xy(69, 115))
		.fade_in(FRONT);

	dep.cmd.batch(CREDITS_FADE_OUT_TIMEOUT)
		.fade_out(FRONT);

	dep.cmd.batch(CREDITS_FADE_IN_TIMEOUT)
		.print(FRONT, "MUSIC", Point::xy(125, 67))
		.print(FRONT, "BY", Point::xy(146, 91))
		.print(FRONT, "PATRICK PHELAN", Point::xy(62, 115))
		.fade_in(FRONT);

	dep.cmd.batch(CREDITS_FADE_OUT_TIMEOUT)
		.fade_out(FRONT);

	dep.cmd.batch(CREDITS_FADE_IN_TIMEOUT)
		.print(FRONT, "PC CONVERSION", Point::xy(69, 43))
		.print(FRONT, "BY", Point::xy(146, 67))
		.print(FRONT, "JON MEDHURST FOR", Point::xy(48, 91))
		.print(FRONT, "CYGNUS SOFTWARE", Point::xy(55, 115))
		.print(FRONT, "ENGINEERING LTD.", Point::xy(52, 139))
		.fade_in(FRONT);

	dep.cmd.batch(CREDITS_FADE_OUT_TIMEOUT)
		.fade_out(FRONT);

	dep.cmd.batch(CREDITS_FADE_IN_TIMEOUT)
		.print(FRONT, "COPYRIGHT 1993", Point::xy(62, 43))
		.print(FRONT, "MAGNETIC FIELDS", Point::xy(55, 67))
		.print(FRONT, "(SOFTWARE DESIGN) LTD.", Point::xy(10, 91))
		.print(FRONT, "GREMLIN GRAPHICS", Point::xy(48, 115))
		.print(FRONT, "SOFTWARE LTD.", Point::xy(73, 139))
		.fade_in(FRONT);

	dep.cmd.batch(CREDITS_FADE_OUT_TIMEOUT)
		.fade_out(FRONT);

	let q1b = dep.arc.get("Q1B")?;

	for step in 1..=36 {
		dep.cmd.batch(Some(50))
			.clear(FRONT)
			.paint(FRONT, draw_a_car(q1b.clone(), step), CAR_SIZE, SCREEN_START)
			.present();
	}

	dep.cmd.batch(Some(50))
		.clear(ALL)
		.draw(BACK, Sprite::from(dep.arc.get("Q1C")?), SCREEN_START)
		.present();

	dep.cmd.batch(Some(50))
		.clear(ALL)
		.draw(BACK, Sprite::from(dep.arc.get("Q1D")?), SCREEN_START)
		.present();

	let q1e = dep.arc.get("Q1E")?;
	let color_ix = *q1e.first().unwrap() as usize;

	dep.cmd.batch(Some(2000))
		.clear(ALL)
		.draw(BACK, Sprite::from(q1e), SCREEN_START)
		.present();

	dep.cmd.batch(Some(4000))
		.fade_out_color(color_ix);

	dep.cmd.batch(None)
		.fade_out(ALL);

	Ok(())
}

fn draw_a_car(data: Vec<u8>, step: usize) -> Box<PaintFn> {
	const SCREEN_WIDTH: usize = 336; // NB: not 320!

	let cx = 256 + (36 - step) * 512;
	let di = 160
		- ((((SCREEN_WIDTH * 170) + 224) / cx) >> 1) + (SCREEN_WIDTH * 64)
		- (((((SCREEN_WIDTH * 118) + 288) / cx) * (SCREEN_WIDTH * 32 + 170)) >> 16) * SCREEN_WIDTH;

	let xx = di % SCREEN_WIDTH;
	let mut y = di / SCREEN_WIDTH;

	Box::new(move |pal: &[u8], c: &mut dyn PaintCanvas| {
		for row in (0..SCREEN_WIDTH * 118).step_by(cx) {
			let offset = (row >> 8) * 224;
			let mut x = xx;
	
			for i in (offset..offset + 224).step_by(cx >> 8) {
				let px = *data.get(i).unwrap() as usize;
	
				if px != 0xFF {
					c.color(pal[px * 3 + 0] << 2, pal[px * 3 + 1] << 2, pal[px * 3 + 2] << 2, 255);
					c.point(Point::xy(x as i32, y as i32));
				}
	
				x += 1;
			}
	
			y += 1;
		}
	})
}

fn show_lotus_logo(dep: &mut Dependencies) -> Result<()> {
	let (q18, pal) = dep.arc.get_with_palette("Q18")?;

	dep.cmd.batch(Some(2000))
		.clear(ALL)
		.palette(pal)
		.draw(BACK, Sprite::from(q18), SCREEN_START)
		.fade_in(ALL);

	dep.cmd.batch(None)
		.fade_out(ALL);

	Ok(())
}

fn show_magazine(dep: &mut Dependencies) -> Result<()> {
	const VIDEO_SIZE: Size = Size::wh(160, 112);
	const VIDEO_POS: Point = Point::xy(136, 38);

	let (v32, mut pal) = dep.arc.get_with_palette("V32")?;

	dep.cmd.batch(None)
		.clear(ALL)
		.palette(pal.clone())
		.draw(BACK, Sprite::from(v32), SCREEN_START);

	fn get_with_leading_pal(arc: &Archive, key: &str, pal: &mut [u8]) -> Result<Vec<u8>> {
		let mut vpal = arc.get(key)?;
		let vdat = vpal.split_off(720);

		let mut iter = pal.iter_mut().skip(16 * 3);

		for b in &vpal {
			*iter.next().unwrap() = *b;
		}

		Ok(vdat)
	}

	const KEYS: [&str; 41] = [
		"V00", "V01", "V02", "V03", "V04", "V05", "V06", "V07", "V08", "V09", "V0A",
		"V0B", "V0C", "V0D", "V0E", "V0F", "V10", "V11", "V12", "V13", "V14", "V15",
		"V16", "V17", "V18", "V19", "V1A", "V1B", "V1C", "V1D", "V1E", "V1F", "V20",
		"V21", "V22", "V23", "V24", "V25", "V26", "V27", "V28"
	];

	for key in KEYS.iter() {
		let dat = get_with_leading_pal(dep.arc, key, &mut pal)?;

		let b = dep.cmd.batch(Some(100))
			.palette(pal.clone())
			.clear(FRONT)
			.draw(FRONT, Sprite::from(dat).with_size(VIDEO_SIZE), VIDEO_POS);

		if key == &"V00" {
			b.fade_in(ALL);
		} else {
			b.present();
		}
	}

	dep.cmd.batch(None)
		.fade_out(FRONT);

	let v33 = get_with_leading_pal(dep.arc, "V33", &mut pal)?;

	dep.cmd.batch(Some(2000))
		.palette(pal)
		.draw(FRONT, Sprite::from(v33).with_size(VIDEO_SIZE), VIDEO_POS)
		.fade_in(FRONT);

	dep.cmd.batch(None)
		.fade_out(ALL)
		.state(GameState::main_menu());

	Ok(())
}
