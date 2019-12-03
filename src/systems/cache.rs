use crate::{
	data::Archive,
	ecs::{
		errors::{Error, Result},
		system::System,
	},
	graphics::{
		font::{Font, CHAR_SET_03, CHAR_SET_04},
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
	pub font_q1a: SpriteFont,
}

impl<'ctx> System<'ctx> for Cache {
	type Dependencies = Dependencies<'ctx>;
	type Error = Error;

	fn create(dep: Self::Dependencies) -> Result<Self> {
		Ok(Self {
			font_c03: Font::from(CHAR_SET_03, dep.arc.get("C03").unwrap()),
			font_c04: Font::from(CHAR_SET_04, dep.arc.get("C04").unwrap()),
			font_q1a: SpriteFont::from(dep.arc.get("Q1A").unwrap()),
		})
	}

	fn debug_name() -> &'static str {
		file!()
	}
}
