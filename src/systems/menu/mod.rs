use crate::graphics::Size;

const MENU_ITEM_SIZE: Size = Size::wh(104, 26);

const FRAME_OFFSET: (u32, u32) = (2, 1); // (x, y) offset (in pixels) of a top left frame

const FRAME_SIZE_ST: Size = Size::wh(95 + 4 + 8, 30 + 4 + 8); // standard (95 x 30)
const _FRAME_SIZE_2R: Size = Size::wh(95 + 4 + 8, 70 + 4 + 8); // 2 rows wide (95 x 70)
const FRAME_SIZE_4R: Size = Size::wh(95 + 4 + 8, 147 + 4 + 8); // 4 rows wide (95 x 147)
const _FRAME_SIZE_3C: Size = Size::wh(303 + 4 + 8, 30 + 4 + 8); // 3 columns wide (303 x 30)

const START_ITEM_POS: (u8, u8) = (0, 1);
const DEFINE_ITEM_POS: (u8, u8) = (4, 2);

mod define;
mod main;

pub use define::Menu as DefineMenu;
pub use main::Menu as MainMenu;
