macro_rules! protection_screen {
	($state:expr, $arc:expr, $font_c03:expr, $input:expr) => {
		let i22 = $arc.get("I22").unwrap();
		let mut i22 = i22.chunks((HELMET_SIZE.width * HELMET_SIZE.height) as usize);

		let helmets = (
			Box::new(Sprite::from(i22.next().unwrap().to_vec()).with_size(HELMET_SIZE)),
			Box::new(Sprite::from(i22.next().unwrap().to_vec()).with_size(HELMET_SIZE)),
		);

		let (i21, pal) = $arc.get_with_palette("I21").unwrap();

		yield Batch::from(vec![
			Command::Palette(pal),
			Command::Draw(Box::new(Sprite::from(i21)), SCREEN_START),
			Command::Draw(helmets.0, point!(141, 13)),
			Command::Draw(helmets.1, point!(141, 73)),
			Command::Print($font_c03, "ENTER CODE FOR WINDOW 47", point!(60, 140)),
			Command::Present,
		]);

		let mut prev_input = $input.clone();

		while *$state == GameState::ProtectionScreen {
			if $input != &prev_input && $input.len() < 4 {
				prev_input = $input.clone();

				yield Batch::from(vec![
					Command::Print($font_c03, $input, point!(150, 165)),
					Command::Present,
					Command::Pop,
				]);
			}

			yield Batch::from(vec![]).sleep(10);
		}
	}
}

macro_rules! show_gremlin {
	($state:expr, $arc:expr) => {
		let (q00, pal) = $arc.get_with_palette("Q00").unwrap();

		yield Batch::from(vec![
			Command::Palette(pal),
			Command::Draw(Box::new(Sprite::from(q00)), SCREEN_START),
			Command::FadeIn,
		]).sleep(200);

		let q01 = $arc.get_series("Q01", SPLASH_SIZE.width * SPLASH_SIZE.height).unwrap();

		for i in &[0usize, 1, 2, 3, 2, 1, 0] {
			if *$state != GameState::Intro { break; }

			yield Batch::from(vec![
				Command::Draw(Box::new(Sprite::from(q01.get(*i).unwrap().to_vec()).with_size(SPLASH_SIZE)), point!(112, 85)),
				Command::Present,
				Command::Pop,
			]).sleep(100);
		}

		for i in &[4usize, 5, 6, 7, 6, 5, 4] {
			if *$state != GameState::Intro { break; }

			yield Batch::from(vec![
				Command::Draw(Box::new(Sprite::from(q01.get(*i).unwrap().to_vec()).with_size(SPLASH_SIZE)), point!(144, 110)),
				Command::Present,
				Command::Pop,
			]).sleep(100);
		}
	}
}

macro_rules! show_magnetic_fields {
	($state:expr, $arc:expr) => {
		let keys = vec![
			"Q02", "Q03", "Q04", "Q05", "Q06", "Q07", "Q08", "Q09", "Q0A", "Q0B", "Q0C", "Q0D",
			"Q0E", "Q0F", "Q10", "Q11", "Q12", "Q13", "Q14", "Q15", "Q16", "Q17",
		];

		let (_, pal) = $arc.get_with_palette(keys.last().unwrap()).unwrap();

		yield Batch::from(vec![
			Command::Palette(pal),
		]);

		for key in keys {
			if *$state != GameState::Intro { break; }

			yield Batch::from(vec![
				Command::Clear,
				Command::Draw(Box::new(Sprite::from($arc.get(key).unwrap())), SCREEN_START),
				Command::Present,
			]).sleep(50);
		}

		yield_if!(*$state == GameState::Intro, Batch::from(vec![]).sleep(500));
	}
}

macro_rules! show_credits {
	($state:expr, $arc:expr, $font_q1a:expr) => {
		let (q19, pal) = $arc.get_with_palette("Q19").unwrap();

		yield_if!(*$state == GameState::Intro, Batch::from(vec![
			Command::Palette(pal),
			Command::Draw(Box::new(Sprite::from(q19)), SCREEN_START),
			Command::FadeIn,
		]).sleep(200));

		yield_if!(*$state == GameState::Intro, Batch::from(vec![
			Command::Print($font_q1a, "A GAME", point!(118, 43)),
			Command::Print($font_q1a, "BY", point!(146, 67)),
			Command::Print($font_q1a, "ANDREW MORRIS", point!(69, 91)),
			Command::Print($font_q1a, "AND", point!(139, 115)),
			Command::Print($font_q1a, "SHAUN SOUTHERN", point!(62, 139)),
			Command::Present,
		]).sleep(1000));

		yield_if!(*$state == GameState::Intro, Batch::from(vec![
			Command::Clear,
			Command::Print($font_q1a, "LEVEL DESIGN", point!(76, 67)),
			Command::Print($font_q1a, "BY", point!(146, 91)),
			Command::Print($font_q1a, "PETER LIGGETT", point!(69, 115)),
			Command::Present,
		]).sleep(1000));

		yield_if!(*$state == GameState::Intro, Batch::from(vec![
			Command::Clear,
			Command::Print($font_q1a, "MUSIC", point!(125, 67)),
			Command::Print($font_q1a, "BY", point!(146, 91)),
			Command::Print($font_q1a, "PATRICK PHELAN", point!(62, 115)),
			Command::Present,
		]).sleep(1000));

		yield_if!(*$state == GameState::Intro, Batch::from(vec![
			Command::Clear,
			Command::Print($font_q1a, "PC CONVERSION", point!(69, 43)),
			Command::Print($font_q1a, "BY", point!(146, 67)),
			Command::Print($font_q1a, "JON MEDHURST FOR", point!(48, 91)),
			Command::Print($font_q1a, "CYGNUS SOFTWARE", point!(55, 115)),
			Command::Print($font_q1a, "ENGINEERING LTD.", point!(52, 139)),
			Command::Present,
		]).sleep(1000));

		yield_if!(*$state == GameState::Intro, Batch::from(vec![
			Command::Clear,
			Command::Print($font_q1a, "COPYRIGHT 1993.", point!(62, 43)),
			Command::Print($font_q1a, "MAGNETIC FIELDS", point!(55, 67)),
			Command::Print($font_q1a, "(SOFTWARE DESIGN) LTD.", point!(10, 91)),
			Command::Print($font_q1a, "GREMLIN GRAPHICS", point!(48, 115)),
			Command::Print($font_q1a, "SOFTWARE LTD.", point!(73, 139)),
			Command::Present,
		]).sleep(1000));
	}
}
