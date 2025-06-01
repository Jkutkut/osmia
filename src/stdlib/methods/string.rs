use super::*;
use regex::Regex;

pub fn module() -> Module {
	Module::new()
	.add_value("lower", Callable::new(1,
		|_, args| Ok(Expr::Str(string_or_fail(&args[0])?.to_lowercase().into())),
		#[cfg(feature = "detailed-dumper")]
		"Returns the string in lowercase"
	).into())
	.add_value("upper", Callable::new(1,
		|_, args| Ok(Expr::Str(string_or_fail(&args[0])?.to_uppercase().into())),
		#[cfg(feature = "detailed-dumper")]
		"Returns the string in uppercase"
	).into())
	.add_value("trim", Callable::new(1,
		|_, args| Ok(Expr::Str(string_or_fail(&args[0])?.trim().into())),
		#[cfg(feature = "detailed-dumper")]
		"Removes whitespaces from sides"
	).into())
	.add_value("capitalize", Callable::new(1,
		|_, args| {
			let mut s = string_or_fail(&args[0])?.chars();
			let mut result = String::new();
			let mut first = true;
			while let Some(c) = s.next() {
				if c.is_alphabetic() && first {
					result.push(c.to_uppercase().next().unwrap());
					first = false;
				} else {
					result.push(c);
					first = !c.is_alphabetic();
				}
			}
			Ok(Expr::Str(result))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns the string capitalized"
	).into())
	.add_value("starts_with", Callable::new(2,
		|_, args| Ok(Expr::Bool(string_or_fail(&args[0])?.starts_with(string_or_fail(&args[1])?).into())),
		#[cfg(feature = "detailed-dumper")]
		"Checks if the string starts with the given prefix"
	).into())
	.add_value("ends_with", Callable::new(2,
		|_, args| Ok(Expr::Bool(string_or_fail(&args[0])?.ends_with(string_or_fail(&args[1])?).into())),
		#[cfg(feature = "detailed-dumper")]
		"Checks if the string ends with the given suffix"
	).into())
	.add_value("ensure_starts_with", Callable::new(2,
		|_, args| {
			let s = string_or_fail(&args[0])?;
			let prefix = string_or_fail(&args[1])?;
			match s.starts_with(prefix) {
				true => Ok(Expr::Str(s.into())),
				false => Ok(Expr::Str(format!("{}{}", prefix, s).into()))
			}
		},
		#[cfg(feature = "detailed-dumper")]
		"Ensures the variable starts with the prefix"
	).into())
	.add_value("ensure_ends_with", Callable::new(2,
		|_, args| {
			let s = string_or_fail(&args[0])?;
			let suffix = string_or_fail(&args[1])?;
			match s.ends_with(suffix) {
				true => Ok(Expr::Str(s.into())),
				false => Ok(Expr::Str(format!("{}{}", s, suffix).into()))
			}
		},
		#[cfg(feature = "detailed-dumper")]
		"Ensures the variable ends with the suffix"
	).into())
	.add_value("index_of", Callable::new(2,
		|_, args| match string_or_fail(&args[0])?.find(string_or_fail(&args[1])?) {
			Some(i) => Ok(Expr::Int(i as i64)),
			None => Ok(Expr::Int(-1))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns the index of the needle in the haystack"
	).into())
	.add_value("last_index_of", Callable::new(2,
		|_, args| match string_or_fail(&args[0])?.rfind(string_or_fail(&args[1])?) {
			Some(i) => Ok(Expr::Int(i as i64)),
			None => Ok(Expr::Int(-1))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns the last index of the needle in the haystack"
	).into())
	// .add_value("left_pad", Callable::new(2,
	// 	|_, args| todo!() // TODO
	// ).into())
	// .add_value("right_pad", Callable::new(2,
	// 	|_, args| todo!() // TODO
	// ).into())
	// .add_value("pad", Callable::new(2,
	// 	|_, args| todo!() // TODO
	// ).into())
	.add_value("match", Callable::new(2,
		|_, args| {
			let s = string_or_fail(&args[0])?;
			let pattern = string_or_fail(&args[1])?;
			let re = match Regex::new(pattern) {
				Ok(re) => re,
				Err(e) => return Err(format!("Invalid regex: {}", e))
			};
			Ok(Expr::Bool(re.is_match(&s)))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns whether the variable matches the regex"
	).into())
	.add_value("replace", Callable::new(3,
		|_, args| {
			let s = string_or_fail(&args[0])?;
			let pattern = match Regex::new(string_or_fail(&args[1])?) {
				Ok(re) => re,
				Err(e) => return Err(format!("Invalid regex: {}", e))
			};
			let repl = string_or_fail(&args[2])?;
			Ok(Expr::Str(pattern.replace(&s, repl).into()))
		},
		#[cfg(feature = "detailed-dumper")]
		"Replaces the needle with the replacement"
	).into())
	.add_value("replace_all", Callable::new(3,
		|_, args| {
			let s = string_or_fail(&args[0])?;
			let pattern = match Regex::new(string_or_fail(&args[1])?) {
				Ok(re) => re,
				Err(e) => return Err(format!("Invalid regex: {}", e))
			};
			let repl = string_or_fail(&args[2])?;
			Ok(Expr::Str(pattern.replace_all(&s, repl).into()))
		},
		#[cfg(feature = "detailed-dumper")]
		"Replaces all needles with the replacement"
	).into())
	.add_value("split", Callable::new(2,
		|_, args| Ok(Expr::Array(
			string_or_fail(&args[0])?
				.split(string_or_fail(&args[1])?)
				.map(|s| Expr::Str(s.into()))
				.collect::<Vec<Expr>>().into()
		)),
		#[cfg(feature = "detailed-dumper")]
		"Splits the string by the separator"
	).into())
	.add_value("substring", Callable::new(3,
		|_, args| {
			let s = string_or_fail(&args[0])?;
			let start = usize_or_fail(&args[1])?;
			let end = usize_or_fail(&args[2])?;
			if start > end {
				return Err(format!("Cannot start after end: {} > {}", start, end));
			}
			Ok(Expr::Str(s[start..end].into()))
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns a substring of the string"
	).into())
	.add_value("truncate", Callable::new(2,
		|_, args| {
			let s = string_or_fail(&args[0])?;
			let len = std::cmp::min(usize_or_fail(&args[1])?, s.len());
			Ok(Expr::Str(s[..len].into()))
		},
		#[cfg(feature = "detailed-dumper")]
		"Truncates the string to the given length"
	).into())
}
