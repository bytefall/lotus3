use anyhow::Result;
use enum_dispatch::enum_dispatch;

mod timer;

pub use self::timer::Timer;

#[enum_dispatch(SystemEnum)]
pub trait System {
	fn update(&mut self) -> Result<()>;
}

#[enum_dispatch]
pub enum SystemEnum {
	Timer,
}
