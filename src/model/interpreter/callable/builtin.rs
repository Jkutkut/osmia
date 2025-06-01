use super::*;

pub type BuiltinArg = fn(intpr: &OsmiaInterpreter<'_>, args: CallableArgs) -> Result<Expr, OsmiaError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Builtin {
	arity: Option<usize>,
	call: BuiltinArg,
	params: Option<Vec<FunctionParam>>,
	#[cfg(feature = "detailed-dumper")]
	description: String,
}

impl Builtin {
	fn raw(
		arity: Option<usize>,
		call: BuiltinArg,
		params: Option<Vec<FunctionParam>>,
		#[cfg(feature = "detailed-dumper")] description: &str
	) -> Self {
		Self {
			arity,
			call,
			params,
			#[cfg(feature = "detailed-dumper")]
			description: description.into(),
		}
	}

	pub fn new(
		arity: usize,
		call: BuiltinArg,
		#[cfg(feature = "detailed-dumper")] description: &str
	) -> Self {
		Self::raw(
			Some(arity), call, None,
			#[cfg(feature = "detailed-dumper")]
			description
		)
	}

	pub fn new_variable_args(
		call: BuiltinArg,
		#[cfg(feature = "detailed-dumper")] description: &str
	) -> Self {
		Self::raw(
			None, call, None,
			#[cfg(feature = "detailed-dumper")]
			description
		)
	}

	pub fn arity(&self) -> Option<usize> {
		self.arity
	}

	pub fn call(&self, intpr: &OsmiaInterpreter<'_>, args: CallableArgs) -> Result<Expr, OsmiaError> {
		(self.call)(intpr, args)
	}

	pub fn params(&self) -> Option<&Vec<FunctionParam>> {
		self.params.as_ref()
	}

	#[cfg(feature = "detailed-dumper")]
	pub fn description(&self) -> &str {
		&self.description
	}
}

impl std::fmt::Display for Builtin {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{ fn (...) => ... }}")
	}
}
