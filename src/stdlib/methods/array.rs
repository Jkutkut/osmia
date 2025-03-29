use super::*;

pub fn module() -> Module {
	Module::new()
	.add_value("sort", Callable::new(1,
		|_, args| Ok(arr_or_fail(&args[0])?.sort()?.into())
	).into())
}
