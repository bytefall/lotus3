use enum_dispatch::enum_dispatch;
use eyre::Result;

mod timer;

pub use timer::Timer;

#[enum_dispatch(SystemEnum)]
pub trait System {
	fn update(&mut self) -> Result<()>;
}

#[enum_dispatch]
pub enum SystemEnum {
	Timer,
}
