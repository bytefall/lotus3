macro_rules! type_list {
	() => { $crate::ecs::hlist::Nil };
	($head:ty, $($tail:ty,)*) => { $crate::ecs::hlist::Cons<$head, type_list!($($tail,)*)> };
}

#[macro_export]
macro_rules! derive_dependencies_from {
	(
		pub struct $name:ident<'ctx> {
			$(
				$dependency_field:ident : $dependency:ty,
			)*
		}
	) => {
		pub struct $name<'ctx> {
			$(pub $dependency_field: $dependency,)*
		}

		impl<'ctx, ContextT, IndicesT>
			$crate::ecs::context::DependenciesFrom<ContextT, IndicesT>
			for $name<'ctx>
		where
			ContextT: $crate::ecs::hlist::PluckList<
				type_list!($($dependency,)*),
				IndicesT,
			>,
		{
			fn dependencies_from(context: ContextT) -> Self {
				let (rest, _) = context.pluck_list();
				$(
					let $crate::ecs::hlist::Cons { head: $dependency_field, tail: rest } = rest;
				)*
				let _ = rest;
				$name { $($dependency_field,)* }
			}
		}
	};
}
