use super::*;

macro_tests!(
	test,
	(
		lambda_01,
		Some("{{ fn () => 42 }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Function,
			Token::Whitespace,
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::Arrow,
			Token::Whitespace,
			Token::new_number("42"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Lambda(Lambda::new(Vec::new(), Expr::Int(42))).into()),
		None
	),
	(
		lambda_02,
		Some("{{ fn (foo) => foo }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Function,
			Token::Whitespace,
			Token::ParentStart,
			Token::new_alpha("foo"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::Arrow,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Lambda(Lambda::new(
			vec![FunctionParam::new("foo".into(), None)],
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("foo".into())
			]).into()
		)).into()),
		None
	),
	(
		lambda_03,
		Some("{{ fn (op1, op2) => op1 + op2 }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Function,
			Token::Whitespace,
			Token::ParentStart,
			Token::new_alpha("op1"),
			Token::Comma,
			Token::Whitespace,
			Token::new_alpha("op2"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::Arrow,
			Token::Whitespace,
			Token::new_alpha("op1"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_alpha("op2"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Lambda(Lambda::new(
			vec![
				FunctionParam::new("op1".into(), None),
				FunctionParam::new("op2".into(), None)
			],
			Binary::new(
				Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("op1".into())
				]).into(),
				BinaryOp::Plus,
				Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("op2".into())
				]).into()
			).into()
		)).into()),
		None
	),
	(
		lambda_04,
		Some("{{ fn (op1=true, op2 = false) => op1 + op2 }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Function,
			Token::Whitespace,
			Token::ParentStart,
			Token::new_alpha("op1"),
			Token::Assign,
			Token::Bool(true),
			Token::Comma,
			Token::Whitespace,
			Token::new_alpha("op2"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::Bool(false),
			Token::ParentEnd,
			Token::Whitespace,
			Token::Arrow,
			Token::Whitespace,
			Token::new_alpha("op1"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_alpha("op2"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Lambda(Lambda::new(
			vec![
				FunctionParam::new(
					"op1".into(),
					Some(Expr::Bool(true)),
				),
				FunctionParam::new(
					"op2".into(),
					Some(Expr::Bool(false)),
				),
			],
			Binary::new(
				Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("op1".into())
				]).into(),
				BinaryOp::Plus,
				Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("op2".into())
				]).into()
			).into()
		)).into()),
		None
	),
	(
		lambda_05,
		Some("{{ fn (...ops) => ops }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Function,
			Token::Whitespace,
			Token::ParentStart,
			Token::Spread,
			Token::new_alpha("ops"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::Arrow,
			Token::Whitespace,
			Token::new_alpha("ops"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Lambda(Lambda::new(
			vec![FunctionParam::new_spread("ops".into())],
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("ops".into())
			]).into()
		)).into()),
		None
	),
	(
		lambda_06,
		Some("{{ fn (foo, ...ops) => foo + ops[0] }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Function,
			Token::Whitespace,
			Token::ParentStart,
			Token::new_alpha("foo"),
			Token::Comma,
			Token::Whitespace,
			Token::Spread,
			Token::new_alpha("ops"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::Arrow,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_alpha("ops"),
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Lambda(Lambda::new(
			vec![
				FunctionParam::new("foo".into(), None),
				FunctionParam::new_spread("ops".into())
			],
			Binary::new(
				Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("foo".into())
				]).into(),
				BinaryOp::Plus,
				Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("ops".into()),
					JsonTreeKeyExpr::Expr(Expr::Int(0))
				]).into()
			).into()
		)).into()),
		None
	)
);
