use super::*;

macro_tests!(
	test,
	(
		foreach01,
		Some("{{for v in [1, 2, 3]}}{{ v }}{{done}}"),
		Some(vec![
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::ArrayStart,
			Token::new_number("1"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("2"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("3"),
			Token::ArrayEnd,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::For(For::new(
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			]),
			Expr::Array(vec![
				Expr::Int(1), Expr::Int(2), Expr::Int(3)
			].into()),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			])).into(),
		))),
		None
		// r#"{}"#,
		// "123"
		// r#"{"v": 3}"#,
		// "123"
	),
	(
		foreach03,
		Some(r#"{{for v in [true, false, null, "hello world"]}}{{ v }}{{done}}"#),
		Some(vec![
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::ArrayStart,
			Token::Bool(true),
			Token::Comma,
			Token::Whitespace,
			Token::Bool(false),
			Token::Comma,
			Token::Whitespace,
			Token::Null,
			Token::Comma,
			Token::Whitespace,
			Token::new_str("hello world"),
			Token::ArrayEnd,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::For(For::new(
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			]),
			Expr::Array(vec![
				Expr::Bool(true), Expr::Bool(false), Expr::Null, Expr::new_str("hello world")
			].into()),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			])).into(),
		))),
		None
		// r#"{"v": 3}"#,
		// "truefalsenullhello world"
	),
	(
		foreach04,
		Some("{{for v in []}}{{ v }}{{done}}"),
		Some(vec![
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::ArrayStart,
			Token::ArrayEnd,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::For(For::new(
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			]),
			Expr::Array(vec![].into()),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			])).into(),
		))),
		None
		// r#"{}"#,
		// ""
	),
	(
		foreach05,
		Some("{{for v in [1 + v, 2, 3]}}{{ v }}{{done}}"),
		Some(vec![
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::ArrayStart,
			Token::new_number("1"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("2"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("3"),
			Token::ArrayEnd,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::For(For::new(
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			]),
			Expr::Array(vec![
				Binary::new(
					Expr::Int(1),
					BinaryOp::Plus,
					Expr::Variable(Variable::from_vec(vec![
						JsonTreeKeyExpr::JsonTreeKey("v".into())
					]))
				).into(),
				Expr::Int(2),
				Expr::Int(3),
			].into()),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			])).into(),
		))),
		None
		// r#"{"v": 3}"#,
		// "423"
	),
	(
		foreachvariable01,
		Some("{{for v in arr}}{{ v }}{{done}}"),
		Some(vec![
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::new_alpha("arr"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::For(For::new(
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			]),
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("arr".into())
			]).into(),
			Stmt::Expr(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			]).into()),
		))),
		None
		// r#"{"arr": [1, 2, 3]}"#,
		// "123"
		// r#"{"arr": [true, "2", null]}"#,
		// " true 2 null"
		// r#"{"arr": []}"#,
		// ""
		// r#"{"arr": [12.3]}"#,
		// "12.3"
	),
	(
		foreach06,
		Some("{{for v in [[1, 2], [3, 4]]}}{{ v[0] }} -- {{ v[1] }},{{done}}"),
		Some(vec![
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::ArrayStart,
			Token::ArrayStart,
			Token::new_number("1"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("2"),
			Token::ArrayEnd,
			Token::Comma,
			Token::Whitespace,
			Token::ArrayStart,
			Token::new_number("3"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("4"),
			Token::ArrayEnd,
			Token::ArrayEnd,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::new_raw(" -- "),
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("1"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::new_raw(","),
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::For(For::new(
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			]),
			Expr::Array(vec![
				Expr::Array(vec![
					Expr::Int(1),
					Expr::Int(2),
				].into()),
				Expr::Array(vec![
					Expr::Int(3),
					Expr::Int(4),
				].into()),
			].into()),
			Stmt::Block(vec![
				Expr::Variable(Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("v".into()),
					JsonTreeKeyExpr::Expr(Expr::Int(0)),
				])).into(),
				Stmt::new_raw(" -- "),
				Expr::Variable(Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("v".into()),
					JsonTreeKeyExpr::Expr(Expr::Int(1)),
				])).into(),
				Stmt::new_raw(","),
			].into()),
		))),
		None
		// "{}",
		// "1 -- 2,3 -- 4,"
	),
	(
		foreach07,
		Some(r#"{{for v in [{"name": "foo"}, {"name": "bar" + extra}]}}{{ v.name }},{{done}}"#),
		Some(vec![
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::ArrayStart,
			Token::ObjectStart,
			Token::new_str("name"),
			Token::Colon,
			Token::Whitespace,
			Token::new_str("foo"),
			Token::ObjectEnd,
			Token::Comma,
			Token::Whitespace,
			Token::ObjectStart,
			Token::new_str("name"),
			Token::Colon,
			Token::Whitespace,
			Token::new_str("bar"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_alpha("extra"),
			Token::ObjectEnd,
			Token::ArrayEnd,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Dot,
			Token::new_alpha("name"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::new_raw(","),
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::For(For::new(
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into()),
			]),
			Expr::Array(vec![
				Expr::Object(vec![
					(Expr::new_str("name"), Expr::Str("foo".into())),
				].into()),
				Expr::Object(vec![
					(Expr::new_str("name"), Binary::new(
						Expr::Str("bar".into()),
						BinaryOp::Plus,
						Expr::Variable(Variable::from_vec(vec![
							JsonTreeKeyExpr::JsonTreeKey("extra".into()),
						]))
					).into()),
				].into()),
			].into()),
			Stmt::Block(vec![
				Expr::Variable(Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("v".into()),
					JsonTreeKeyExpr::JsonTreeKey("name".into()),
				])).into(),
				Stmt::new_raw(","),
			].into()),
		))),
		None
		// r#"{"extra": 12}"#,
		// "foo,bar12,"
	),
	(
		nested,
		Some("{{for arr in matrix}}{{for cell in arr}}{{print cell}}{{done}}{{done}}"),
		Some(vec![
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("arr"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::new_alpha("matrix"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("cell"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::new_alpha("arr"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::Print,
			Token::Whitespace,
			Token::new_alpha("cell"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::For(For::new(
			Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("arr".into()),
			]),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("matrix".into()),
			])),
			Stmt::For(For::new(
				Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("cell".into()),
				]),
				Expr::Variable(Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("arr".into()),
				])),
				Stmt::Print(Print::new(Expr::Variable(Variable::from_vec(vec![
					JsonTreeKeyExpr::JsonTreeKey("cell".into())
				])))),
			)),
		))),
		None
	)
);
