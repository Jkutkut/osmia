use crate::macro_tests;
use crate::types::{
	Ctx,
	OsmiaError,
};
use crate::model::{
	expr::Expr,
	interpreter::Callable,
	ctx::{
		JsonTreeKey,
	},
};

fn get_expr(ctx: &mut Ctx, key: &str) -> Result<Expr, OsmiaError> {
	Ok(ctx.get(&mut JsonTreeKey::from(key).iter())?.try_into()?)
}

fn get_ft(ctx: &mut Ctx, key: &str) -> Result<Callable, OsmiaError> {
	ctx.get_callable(&mut JsonTreeKey::from(key).iter())
}

fn check_pieces(
	text: &str,
	pieces: Vec<&str>,
) {
	println!("Checking {:?} in {:?}", pieces, text);
	let text = text.to_lowercase();
	for piece in pieces {
		assert!(text.contains(&piece.to_lowercase()));
	}
}

fn test_constant(
	key: &str,
	expected: Result<Expr, Vec<&str>>,
) {
	let mut ctx = Ctx::new();
	match (get_expr(&mut ctx, key), expected) {
		(Ok(expr), Ok(expected)) => assert_eq!(expr, expected),
		(Err(err), Err(error_pieces)) => check_pieces(&err, error_pieces),
		(r, e) => panic!("Constant should return {:?} but returned {:?}", e, r),
	}
}

fn test_callable(
	ft: &str,
	args: Vec<Expr>,
	expected: Result<Expr, Vec<&str>>,
) {
	let mut ctx = Ctx::new();
	let ft: Callable = match get_ft(&mut ctx, ft) {
		Ok(f) => f,
		Err(err) => match expected {
			Ok(expr) => panic!("Callable should return {:?} but returned {:?}", expr, err),
			Err(error_pieces) => return check_pieces(&err, error_pieces),
		},
	};
	match (ft.call(&mut ctx, &args), expected) {
		(Ok(expr), Ok(expected)) => assert_eq!(expr, expected),
		(Err(err), Err(error_pieces)) => check_pieces(&err, error_pieces),
		(r, e) => panic!("Callable should return {:?} but returned {:?}", e, r),
	}
}

macro_tests!(
	test_constant,
	(pi, "math.PI", Ok(Expr::Float(3.141592653589793))),
	(e, "math.E", Ok(Expr::Float(2.718281828459045))),

	(invalid_const_01, "math.G", Err(vec!["not", "found"])),
);

macro_tests!(
	test_callable,
	(sqrt_01, "math.sqrt", vec![Expr::Float(4.0)], Ok(Expr::Float(2.0))),
	(sqrt_02, "math.sqrt", vec![Expr::Int(16)], Ok(Expr::Float(4.0))),
	(pow_01, "math.pow", vec![Expr::Float(2.0), Expr::Float(2.0)], Ok(Expr::Float(4.0))),
	(pow_02, "math.pow", vec![Expr::Float(2.0), Expr::Int(3)], Ok(Expr::Float(8.0))),

	(invalid_01, "sqrt", vec![], Err(vec!["not", "found"])),
	(invalid_02, "math.invalid_sqrt", vec![], Err(vec!["not", "found"])),
	(invalid_03, "math", vec![], Err(vec!["not", "callable"])),
);
