use crate::graphics::{PaintCanvas, Point, Size};

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
	c.point(Point::xy(2, 1));
	c.point(Point::xy(1, 2));
	c.point(Point::xy(3, 2));
	c.point(Point::xy(2, 3));
	// bottom left
	c.point(Point::xy(2, size.height as i32 - 4));
	c.point(Point::xy(1, size.height as i32 - 3));
	c.point(Point::xy(3, size.height as i32 - 3));
	c.point(Point::xy(2, size.height as i32 - 2));
	// upper right
	c.point(Point::xy(size.width as i32 - 3, 1));
	c.point(Point::xy(size.width as i32 - 4, 2));
	c.point(Point::xy(size.width as i32 - 2, 2));
	c.point(Point::xy(size.width as i32 - 3, 3));
	// bottom right
	c.point(Point::xy(size.width as i32 - 3, size.height as i32 - 4));
	c.point(Point::xy(size.width as i32 - 4, size.height as i32 - 3));
	c.point(Point::xy(size.width as i32 - 2, size.height as i32 - 3));
	c.point(Point::xy(size.width as i32 - 3, size.height as i32 - 2));

	// red
	c.color(162, 0, 0, 255);
	c.point(Point::xy(2, 2)); // upper left
	c.point(Point::xy(size.width as i32 - 3, 2)); // upper right
	c.point(Point::xy(size.width as i32 - 3, size.height as i32 - 3)); // bottom right
	c.point(Point::xy(2, size.height as i32 - 3)); // bottom left

	c.line(Point::xy(1, 3), Point::xy(1, size.height as i32 - 4)); // left vertical
	c.line(Point::xy(3, 1), Point::xy(size.width as i32 - 4, 1)); // upper horizontal
	c.line(Point::xy(size.width as i32 - 2, 3), Point::xy(size.width as i32 - 2, size.height as i32 - 4)); // right vertical
	c.line(Point::xy(3, size.height as i32 - 2), Point::xy(size.width as i32 - 4, size.height as i32 - 2)); // bottom horizontal

	// black
	c.color(0, 0, 0, 255);
	// upper left
	c.point(Point::xy(1, 1));
	c.point(Point::xy(3, 3));
	// upper right
	c.point(Point::xy(size.width as i32 - 2, 1));
	c.point(Point::xy(size.width as i32 - 4, 3));
	// bottom right
	c.point(Point::xy(size.width as i32 - 4, size.height as i32 - 4));
	c.point(Point::xy(size.width as i32 - 2, size.height as i32 - 2));
	// bottom left
	c.point(Point::xy(3, size.height as i32 - 4));
	c.point(Point::xy(1, size.height as i32 - 2));

	c.line(Point::xy(0, 2), Point::xy(0, size.height as i32 - 3)); // left 1
	c.line(Point::xy(2, 4), Point::xy(2, size.height as i32 - 5)); // left 2
	c.line(Point::xy(2, 0), Point::xy(size.width as i32 - 3, 0)); // top 1
	c.line(Point::xy(4, 2), Point::xy(size.width as i32 - 5, 2)); // top 2
	c.line(Point::xy(size.width as i32 - 3, 4), Point::xy(size.width as i32 - 3, size.height as i32 - 5)); // right 1
	c.line(Point::xy(size.width as i32 - 1, 2), Point::xy(size.width as i32 - 1, size.height as i32 - 3)); // right 2
	c.line(Point::xy(4, size.height as i32 - 3), Point::xy(size.width as i32 - 5, size.height as i32 - 3)); // bottom 1
	c.line(Point::xy(2, size.height as i32 - 1), Point::xy(size.width as i32 - 3, size.height as i32 - 1)); // bottom 2
}

const START_ITEM_POS: (u8, u8) = (0, 1);
const DEFINE_ITEM_POS: (u8, u8) = (4, 2);

mod define;
mod main;

pub use define::Menu as DefineMenu;
pub use main::Menu as MainMenu;
