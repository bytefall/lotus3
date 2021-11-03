use eyre::Result;
use std::marker::PhantomData;

pub trait System<'context>: Sized + 'context {
	type Dependencies;

	fn debug_name() -> &'static str;

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

	fn debug_name() -> &'static str;

	fn create(dependencies: Self::Dependencies) -> Self;

	#[inline]
	fn setup(&mut self, _dependencies: Self::Dependencies) {}

	#[inline]
	fn update(&mut self, _dependencies: Self::Dependencies) {}

	#[inline]
	fn teardown(&mut self, _dependencies: Self::Dependencies) {}

	#[inline]
	fn destroy(self, _dependencies: Self::Dependencies) {}
}

impl<'context, SystemT> System<'context> for SystemT
where
	Self: InfallibleSystem<'context>,
{
	type Dependencies = <Self as InfallibleSystem<'context>>::Dependencies;

	#[inline]
	fn debug_name() -> &'static str {
		<Self as InfallibleSystem>::debug_name()
	}

	#[inline]
	fn create(dependencies: Self::Dependencies) -> Result<Self> {
		Ok(<Self as InfallibleSystem>::create(dependencies))
	}

	#[inline]
	fn setup(&mut self, dependencies: Self::Dependencies) -> Result<()> {
		<Self as InfallibleSystem>::setup(self, dependencies);

		Ok(())
	}

	#[inline]
	fn update(&mut self, dependencies: Self::Dependencies) -> Result<()> {
		<Self as InfallibleSystem>::update(self, dependencies);

		Ok(())
	}

	#[inline]
	fn teardown(&mut self, dependencies: Self::Dependencies) -> Result<()> {
		<Self as InfallibleSystem>::teardown(self, dependencies);

		Ok(())
	}

	#[inline]
	fn destroy(self, dependencies: Self::Dependencies) -> Result<()> {
		<Self as InfallibleSystem>::destroy(self, dependencies);

		Ok(())
	}
}

pub struct BoundSystem<SystemT>(PhantomData<SystemT>);
