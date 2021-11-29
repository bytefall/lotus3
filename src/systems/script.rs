use crate::{
	data::Archive,
	ecs::system::InfallibleSystem,
	game::{
		script::{Command, CommandSequence, Layer},
		state::GameFlow,
	},
	graphics::{Canvas, Color, FadeType},
	systems::{Cache, Timer, Window},
};

derive_dependencies_from! {
	pub struct Dependencies<'ctx> {
		arc: &'ctx Archive,
		cache: &'ctx Cache,
		cmd: &'ctx mut CommandSequence,
		timer: &'ctx mut Timer,
		flow: &'ctx mut GameFlow,
		win: &'ctx mut Window,
	}
}

#[derive(Default)]
pub struct Script {
	back: Option<Canvas>,
	front: Option<Canvas>,
}

impl<'ctx> InfallibleSystem<'ctx> for Script {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Self {
		Self::default()
	}

	fn update(&mut self, dep: Self::Dependencies) {
		// quit when there is no batch to process
		let batch = if let Some(batch) = dep.cmd.pop() {
			batch
		} else {
			return;
		};

		for cmd in batch.commands {
			match cmd {
				Command::Palette(pal) => {
					dep.win.palette = pal;
				}
				Command::Draw(target, sprite, pos) => match target {
					Some(Layer::Front) => {
						self.front.get_or_insert_with(Canvas::default).draw(&sprite, &dep.win.palette, pos);
					}
					Some(Layer::Back) => {
						self.back.get_or_insert_with(Canvas::default).draw(&sprite, &dep.win.palette, pos);
					}
					None => {
						dep.win.draw(&sprite, pos);
					}
				}
				Command::Paint(target, paint_fn) => match target {
					Some(Layer::Front) => {
						let front = self.front.get_or_insert_with(Canvas::default);

						paint_fn(front, &dep.win.palette);
					}
					Some(Layer::Back) => {
						let back = self.back.get_or_insert_with(Canvas::default);

						paint_fn(back, &dep.win.palette);
					}
					None => {
						let mut canvas = Canvas::default();
						paint_fn(&mut canvas, &dep.win.palette);

						dep.win.blit(&canvas);
					}
				}
				Command::Print(target, text, pos) => match target {
					Some(Layer::Front) => {
						self.front.get_or_insert_with(Canvas::default).print(text, &dep.cache.font_q1a, &dep.win.palette, pos);
					}
					Some(Layer::Back) => {
						self.back.get_or_insert_with(Canvas::default).print(text, &dep.cache.font_q1a, &dep.win.palette, pos);
					}
					None => {
						dep.win.print(text, &dep.cache.font_q1a, pos);
					}
				}
				Command::Present => {
					self.back.as_ref().map_or((), |c| dep.win.blit(c));
					self.front.as_ref().map_or((), |c| dep.win.blit(c));

					dep.win.present();
				}
				Command::Clear(target) => match target {
					Some(Layer::Front) => {
						self.front = None;
					}
					Some(Layer::Back) => {
						self.back = None;
					}
					None => {
						self.front = None;
						self.back = None;
					}
				}
				Command::FadeIn(target) => match target {
					Some(Layer::Front) => {
						dep.win.fade_only(FadeType::In, self.back.as_ref().unwrap(), self.front.as_ref().unwrap());
					}
					Some(Layer::Back) => {
						self.back.as_ref().map_or((), |c| dep.win.blit(c));

						dep.win.fade_in();
					}
					None => {
						self.back.as_ref().map_or((), |c| dep.win.blit(c));
						self.front.as_ref().map_or((), |c| dep.win.blit(c));

						dep.win.fade_in();
					}
				}
				Command::FadeOut(target) => match target {
					Some(Layer::Front) => {
						dep.win.fade_only(FadeType::Out, self.back.as_ref().unwrap(), self.front.as_ref().unwrap());
					}
					Some(Layer::Back) => {
						self.back.as_ref().map_or((), |c| dep.win.blit(c));

						dep.win.fade_out();
					}
					None => {
						self.back.as_ref().map_or((), |c| dep.win.blit(c));
						self.front.as_ref().map_or((), |c| dep.win.blit(c));

						dep.win.fade_out();
					}
				}
				Command::FadeOutByColor(ix) => {
					let color = dep.win.palette
						.get(ix * 3..ix * 3 + 3)
						.map(|rgb| Color::rgb(rgb[0] << 2, rgb[1] << 2, rgb[2] << 2));

					dep.win.fade_out_by_color(color.unwrap());
				}
				Command::State(state) => {
					dep.flow.set(state);
				}
			}
		}

		if let Some(duration) = batch.sleep {
			dep.timer.sleep(duration);
		}
	}
}
