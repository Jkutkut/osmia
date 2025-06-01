use super::*;

/// # math module
/// ## Constants:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// osmia.run_code("{{ math.PI }}").unwrap();
/// osmia.run_code("{{ math.E }}").unwrap();
/// ```
///
/// ## Functions:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// assert_eq!(osmia.run_code("{{ math.abs(-1) }}").unwrap(), "1".to_string());
/// assert_eq!(osmia.run_code("{{ math.ceil(1.1) }}").unwrap(), "2".to_string());
/// assert_eq!(osmia.run_code("{{ math.floor(1.9) }}").unwrap(), "1".to_string());
/// assert_eq!(osmia.run_code("{{ math.pow(2, 3) }}").unwrap(), "8".to_string());
/// assert_eq!(osmia.run_code("{{ math.sqrt(4) }}").unwrap(), "2".to_string());
/// assert_eq!(osmia.run_code("{{ math.round(1.5) }}").unwrap(), "2".to_string());
/// assert_eq!(osmia.run_code("{{ math.max(1, 2) }}").unwrap(), "2".to_string());
/// assert_eq!(osmia.run_code("{{ math.min(1, 2) }}").unwrap(), "1".to_string());
/// ```
///
/// ## Sub modules:
/// There are no sub modules for this module
pub fn module() -> Module {
	Module::new()
	// Constants
	.add_value("PI", CtxValue::Float(std::f64::consts::PI))
	.add_value("E", CtxValue::Float(std::f64::consts::E))
	// Methods
	.add_value("abs", Callable::new(
		1,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.abs()))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns the absolute value of the number"
	).into())
	.add_value("ceil", Callable::new(
		1,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.ceil()))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns the largest integer greater than or equal to the number"
	).into())
	.add_value("floor", Callable::new(
		1,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.floor()))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns the smallest integer less than or equal to the number"
	).into())
	.add_value("pow", Callable::new(
		2,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.powf(args[1].to_float()?)))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns arg1 raised to the power of arg2"
	).into())
	.add_value("sqrt", Callable::new(
		1,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.sqrt()))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns the square root of the number"
	).into())
	.add_value("round", Callable::new(
		1,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.round()))
		},
		#[cfg(feature = "detailed-dumper")]
		"Rounds the number to the nearest integer"
	).into())
	.add_value("max", Callable::new(
		2,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.max(args[1].to_float()?)))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns the larger of two numbers"
	).into())
	.add_value("min", Callable::new(
		2,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.min(args[1].to_float()?)))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns the smaller of two numbers"
	).into())
}
