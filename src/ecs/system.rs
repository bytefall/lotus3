use eyre::Result;
use std::marker::PhantomData;

pub trait System<'context>: Sized + 'context {
	type Dependencies;

	fn create(dep: Self::Dependencies) -> Result<Self>;

	#[inline]
	fn setup(&mut self, _dep: Self::Dependencies) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn update(&mut self, _dep: Self::Dependencies) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn teardown(&mut self, _dep: Self::Dependencies) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn destroy(self, _dep: Self::Dependencies) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn bind() -> BoundSystem<Self> {
		BoundSystem(PhantomData)
	}
}

pub trait InfallibleSystem<'context>: Sized + 'context {
	type Dependencies;

	fn create(dep: Self::Dependencies) -> Self;

	#[inline]
	fn setup(&mut self, _dep: Self::Dependencies) {}

	#[inline]
	fn update(&mut self, _dep: Self::Dependencies) {}

	#[inline]
	fn teardown(&mut self, _dep: Self::Dependencies) {}

	#[inline]
	fn destroy(self, _dep: Self::Dependencies) {}
}

impl<'context, SystemT> System<'context> for SystemT
where
	Self: InfallibleSystem<'context>,
{
	type Dependencies = <Self as InfallibleSystem<'context>>::Dependencies;

	#[inline]
	fn create(dep: Self::Dependencies) -> Result<Self> {
		Ok(<Self as InfallibleSystem>::create(dep))
	}

	#[inline]
	fn setup(&mut self, dep: Self::Dependencies) -> Result<()> {
		<Self as InfallibleSystem>::setup(self, dep);

		Ok(())
	}

	#[inline]
	fn update(&mut self, dep: Self::Dependencies) -> Result<()> {
		<Self as InfallibleSystem>::update(self, dep);

		Ok(())
	}

	#[inline]
	fn teardown(&mut self, dep: Self::Dependencies) -> Result<()> {
		<Self as InfallibleSystem>::teardown(self, dep);

		Ok(())
	}

	#[inline]
	fn destroy(self, dep: Self::Dependencies) -> Result<()> {
		<Self as InfallibleSystem>::destroy(self, dep);

		Ok(())
	}
}

pub struct BoundSystem<SystemT>(PhantomData<SystemT>);
