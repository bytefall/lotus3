use crate::graphics::{PaintCanvas, Size};

const MENU_ITEM_SIZE: Size = Size::wh(104, 26);

const FRAME_OFFSET: (u32, u32) = (2, 1);
const FRAME_BORDER: u32 = 3;

const FRAME_SIZE_ST: Size = Size::wh(95 + 4 + 8, 30 + 4 + 8); // standard (95 x 30)
const _FRAME_SIZE_2R: Size = Size::wh(95 + 4 + 8, 70 + 4 + 8); // 2 rows wide (95 x 70)
const FRAME_SIZE_4R: Size = Size::wh(95 + 4 + 8, 147 + 4 + 8); // 4 rows wide (95 x 147)
const _FRAME_SIZE_3C: Size = Size::wh(303 + 4 + 8, 30 + 4 + 8); // 3 columns wide (303 x 30)

fn build_frame(size: Size, c: &mut dyn PaintCanvas) {
	// maroon
	c.color(93, 0, 0, 255);
	// upper left
	c.point((2, 1).into());
	c.point((1, 2).into());
	c.point((3, 2).into());
	c.point((2, 3).into());
	// bottom left
	c.point((2, size.height as i32 - 4).into());
	c.point((1, size.height as i32 - 3).into());
	c.point((3, size.height as i32 - 3).into());
	c.point((2, size.height as i32 - 2).into());
	// upper right
	c.point((size.width as i32 - 3, 1).into());
	c.point((size.width as i32 - 4, 2).into());
	c.point((size.width as i32 - 2, 2).into());
	c.point((size.width as i32 - 3, 3).into());
	// bottom right
	c.point((size.width as i32 - 3, size.height as i32 - 4).into());
	c.point((size.width as i32 - 4, size.height as i32 - 3).into());
	c.point((size.width as i32 - 2, size.height as i32 - 3).into());
	c.point((size.width as i32 - 3, size.height as i32 - 2).into());

	// red
	c.color(162, 0, 0, 255);
	c.point((2, 2).into()); // upper left
	c.point((size.width as i32 - 3, 2).into()); // upper right
	c.point((size.width as i32 - 3, size.height as i32 - 3).into()); // bottom right
	c.point((2, size.height as i32 - 3).into()); // bottom left

	c.line((1, 3).into(), (1, size.height as i32 - 4).into()); // left vertical
	c.line((3, 1).into(), (size.width as i32 - 4, 1).into()); // upper horizontal
	c.line((size.width as i32 - 2, 3).into(), (size.width as i32 - 2, size.height as i32 - 4).into()); // right vertical
	c.line((3, size.height as i32 - 2).into(), (size.width as i32 - 4, size.height as i32 - 2).into()); // bottom horizontal

	// black
	c.color(0, 0, 0, 255);
	// upper left
	c.point((1, 1).into());
	c.point((3, 3).into());
	// upper right
	c.point((size.width as i32 - 2, 1).into());
	c.point((size.width as i32 - 4, 3).into());
	// bottom right
	c.point((size.width as i32 - 4, size.height as i32 - 4).into());
	c.point((size.width as i32 - 2, size.height as i32 - 2).into());
	// bottom left
	c.point((3, size.height as i32 - 4).into());
	c.point((1, size.height as i32 - 2).into());

	c.line((0, 2).into(), (0, size.height as i32 - 3).into()); // left 1
	c.line((2, 4).into(), (2, size.height as i32 - 5).into()); // left 2
	c.line((2, 0).into(), (size.width as i32 - 3, 0).into()); // top 1
	c.line((4, 2).into(), (size.width as i32 - 5, 2).into()); // top 2
	c.line((size.width as i32 - 3, 4).into(), (size.width as i32 - 3, size.height as i32 - 5).into()); // right 1
	c.line((size.width as i32 - 1, 2).into(), (size.width as i32 - 1, size.height as i32 - 3).into()); // right 2
	c.line((4, size.height as i32 - 3).into(), (size.width as i32 - 5, size.height as i32 - 3).into()); // bottom 1
	c.line((2, size.height as i32 - 1).into(), (size.width as i32 - 3, size.height as i32 - 1).into()); // bottom 2
}

const START_ITEM_POS: (u8, u8) = (0, 1);
const DEFINE_ITEM_POS: (u8, u8) = (4, 2);

mod define;
mod main;

pub use define::Menu as DefineMenu;
pub use main::Menu as MainMenu;
