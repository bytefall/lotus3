macro_rules! main_menu {
	($state:expr, $arc:expr, $font_c04:expr) => {
		let (i14, pal) = $arc.get_with_palette("I14").unwrap();
		let i15 = $arc.get_series("I15", MENU_ITEM_SIZE.width * MENU_ITEM_SIZE.height).unwrap();

		//let i16 = $arc.get("I16").unwrap(); // define
		//let i17 = $arc.get("I17").unwrap(); // RECS
		// I18
		//let i19 = $arc.get("I19").unwrap(); // control
		// I1A
		//let i1b = $arc.get("I1B").unwrap(); // keyboard
		//let i1f = $arc.get("I1F").unwrap(); // mouse
		//let i20 = $arc.get("I20").unwrap(); // sound

		yield Batch::from(vec![
			Command::Palette(pal),
			Command::Draw(Box::new(Sprite::from(i14)), SCREEN_START),
			Command::Print($font_c04, "PLAYER 1", point!(13, 21)), // P1 name
			Command::Print($font_c04, "PLAYER 2", point!(221, 21)), // P2 name
			Command::Print($font_c04, "VBJD D   -99", point!(117, 177)), // Code
			Command::Draw(Box::new(Sprite::from(i15.get(0 + 1).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)), point!(6, 52)), // P1 gears
			Command::Draw(Box::new(Sprite::from(i15.get(2 + 0).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)), point!(6, 91)), // P1 accelerate
			Command::Draw(Box::new(Sprite::from(i15.get(0 + 0).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)), point!(214, 52)), // P2 gears
			Command::Draw(Box::new(Sprite::from(i15.get(2 + 1).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)), point!(214, 91)), // P2 accelerate
			Command::Draw(Box::new(Sprite::from(i15.get(6 + 1).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)), point!(110, 52)), // Game
			Command::Draw(Box::new(Sprite::from(i15.get(10 + 0).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)), point!(110, 91)), // Course
			Command::Draw(Box::new(Sprite::from(i15.get(8 + 0).unwrap().to_vec()).with_size(MENU_ITEM_SIZE)), point!(110, 130)), // Players
			Command::FadeIn,
		]);

		while *$state == GameState::MainMenu {
			yield Batch::from(vec![]).sleep(10);
		}
	}
}
