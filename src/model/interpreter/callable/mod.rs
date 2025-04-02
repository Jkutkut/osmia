use std::fmt::Debug;

use crate::OsmiaInterpreter;
use crate::types::{
	OsmiaError,
};
use crate::model::{
	stmt::{
		Stmt,
		Function,
	},
	expr::{
		Expr,
		Lambda,
		FunctionParam,
	},
};

mod callable;
mod builtin;
mod lambda_callable;
mod function_callable;

pub use callable::Callable;
pub use builtin::{
	Builtin,
	BuiltinArg,
};
pub use lambda_callable::LambdaCallable;
pub use function_callable::FunctionCallable;

type CallableArgs<'c> = &'c Vec<Expr>;
