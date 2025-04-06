use super::*;
use std::cmp::Ordering;

pub fn module() -> Module {
	Module::new()
	.add_value("sort", Callable::new(1,
		|_, args| Ok(arr_or_fail(&args[0])?.sort()?.into())
	).into())
	.add_value("sort_by", Callable::new(2,
		|intpr, args| {
			let arr = arr_or_fail(&args[0])?;
			let func = callable_or_fail(&args[1])?;
			if func.arity() != Some(2) {
				return Err("sort_by function must accept exactly 2 arguments".into());
			}
			let sorted = arr.sort_by(|a, b| {
				let args = vec![a.clone().into(), b.clone().into()];
				match func.call(intpr, &args) {
					Err(r) => todo!("can not compare {:?} and {:?} => {:?}", a, b, r),
					Ok(e)	=> match intpr.visit_expr(&e) {
						Ok(Expr::Int(i)) => match i {
							0 => Ordering::Equal,
							i => if i < 0 { Ordering::Less } else { Ordering::Greater },
						},
						Ok(Expr::Bool(b)) => if b { Ordering::Less } else { Ordering::Greater },
						Ok(_) => Ordering::Equal,
						Err(_) => Ordering::Equal,
					}
				}
			});
			Ok(sorted.into())
		}
	).into())
	.add_value("map", Callable::new(2,
		|intpr, args| {
			let arr = arr_or_fail(&args[0])?;
			let func = callable_or_fail(&args[1])?;
			if func.arity() != Some(1) {
				return Err("map function must accept exactly 1 argument".into());
			}
			let result = arr.iter()
				.map(|e| Ok(intpr.visit_expr(
					&func.call(intpr, &vec![e.clone().into()])?
				)?))
				.collect::<Result<Vec<Expr>, OsmiaError>>()?;
			Ok(Array::new(result).into())
		}
	).into())
}
