use eyre::Result;

use crate::{
	data::Archive,
	ecs::system::System,
	graphics::{
		font::{Font, CHAR_SET_03, CHAR_SET_04, CHAR_SET_06},
		SpriteFont,
	},
};

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		arc: &'ctx Archive,
	}
}

pub struct Cache {
	pub font_c03: Font,
	pub font_c04: Font,
	pub font_c06: Font,
	pub font_q1a: SpriteFont,
}

impl<'ctx> System<'ctx> for Cache {
	type Dependencies = Dependencies<'ctx>;

	fn create(dep: Self::Dependencies) -> Result<Self> {
		Ok(Self {
			font_c03: Font::from(CHAR_SET_03, dep.arc.get("C03")?),
			font_c04: Font::from(CHAR_SET_04, dep.arc.get("C04")?),
			font_c06: Font::from(CHAR_SET_06, dep.arc.get("C06")?),
			font_q1a: SpriteFont::from(dep.arc.get("Q1A")?),
		})
	}
}
