use generational_arena::Index;

use crate::{
	data::Archive,
	ecs::system::InfallibleSystem,
	game::{
		script::{Command, CommandSequence, Layer},
		state::GameFlow,
	},
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

pub struct Script {
	front_ids: Vec<Index>,
	back_ids: Vec<Index>,
}

impl<'ctx> InfallibleSystem<'ctx> for Script {
	type Dependencies = Dependencies<'ctx>;

	fn create(_: Self::Dependencies) -> Self {
		Self {
			front_ids: Vec::new(),
			back_ids: Vec::new(),
		}
	}

	fn update(&mut self, dep: Self::Dependencies) {
		// quit when there is no batch to process
		let batch = if let Some(batch) = dep.cmd.pop() {
			batch
		} else {
			if !self.front_ids.is_empty() {
				self.front_ids.clear();
			}

			if !self.back_ids.is_empty() {
				self.back_ids.clear();
			}

			return;
		};

		for cmd in batch.commands {
			match cmd {
				Command::Palette(pal) => {
					dep.win.palette = pal;
				}
				Command::Draw(target, sprite, pos) => {
					let id = dep.win.draw(&sprite).show(pos).id;

					match target {
						Some(Layer::Front) => self.front_ids.push(id),
						Some(Layer::Back) => self.back_ids.push(id),
						None => (),
					}
				}
				Command::Paint(target, foo, size, pos) => {
					let id = dep.win.paint(size, foo).show(pos).id;

					match target {
						Some(Layer::Front) => self.front_ids.push(id),
						Some(Layer::Back) => self.back_ids.push(id),
						None => (),
					}
				}
				Command::Print(target, text, pos) => {
					let id = dep.win.print(&dep.cache.font_q1a, text).show(pos).id;

					match target {
						Some(Layer::Front) => self.front_ids.push(id),
						Some(Layer::Back) => self.back_ids.push(id),
						None => (),
					}
				}
				Command::Present => {
					dep.win.present();
				}
				Command::Clear(target) => match target {
					Some(Layer::Front) => {
						dep.win.remove_only(&self.front_ids);

						self.front_ids.clear();
					}
					Some(Layer::Back) => {
						dep.win.remove_only(&self.back_ids);

						self.back_ids.clear();
					}
					None => {
						dep.win.clear();
						dep.win.free();

						self.front_ids.clear();
						self.back_ids.clear();
					}
				},
				Command::FadeIn(target) => match target {
					Some(Layer::Front) => dep.win.fade_in_only(&self.front_ids),
					Some(Layer::Back) => dep.win.fade_in_only(&self.back_ids),
					None => dep.win.fade_in(),
				},
				Command::FadeOut(target) => {
					// remove entities once they are faded out
					match target {
						Some(Layer::Front) => {
							dep.win.fade_out_only(&self.front_ids);
							dep.win.remove_only(&self.front_ids);

							self.front_ids.clear();
						}
						Some(Layer::Back) => {
							dep.win.fade_out_only(&self.back_ids);
							dep.win.remove_only(&self.back_ids);

							self.back_ids.clear();
						}
						None => {
							dep.win.fade_out();
							dep.win.free();

							self.front_ids.clear();
							self.back_ids.clear();
						}
					}
				}
				Command::FadeOutByColorIndex(ix) => {
					dep.win.fade_out_by_color_ix(ix);
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
