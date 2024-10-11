use crate::constants::VERSION;
use crate::types::Ctx;
use crate::model::ctx::{
	CtxValue,
	JsonTreeKey,
	lib::Module,
};
use crate::model::interpreter::Callable;

mod math;

use crate::model::expr::Expr;

pub fn import(ctx: &mut Ctx) {
	ctx.set(
		&mut JsonTreeKey::from("_OSMIA_VERSION").iter(),
		CtxValue::Str(VERSION.into()).into()
	).unwrap();
	ctx.set(
		&mut JsonTreeKey::from("math").iter(),
		math::module().into()
	).unwrap();
}
