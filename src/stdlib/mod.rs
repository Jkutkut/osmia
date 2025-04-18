use crate::constants::VERSION;
use crate::types::Ctx;
use crate::model::ctx::{
	CtxValue,
	JsonTreeKey,
	lib::Module,
};
use crate::model::interpreter::Callable;
use crate::constants::METHOD_CTX_LOCATION;
use crate::model::interpreter::MethodExpression;
use crate::model::interpreter::callable::BuiltinArg;

mod math;
mod methods;

use crate::model::expr::Expr;

/// # stdlib module
/// ## Constants:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// osmia.run_code("{{ _OSMIA_VERSION }}").unwrap();
/// ```
///
/// ## Functions:
/// There are no functions in this module
///
/// ## Sub modules:
/// - [math](./math/fn.module.html)
/// - [methods](./methods/fn.module.html)
pub fn import(ctx: &mut Ctx) {
	ctx.set(
		&JsonTreeKey::from("_OSMIA_VERSION"),
		CtxValue::Str(VERSION.into()).into()
	).unwrap();
	ctx.set(
		&JsonTreeKey::from(METHOD_CTX_LOCATION),
		methods::module().into()
	).unwrap();
	ctx.set(
		&JsonTreeKey::from("math"),
		math::module().into()
	).unwrap();
}
