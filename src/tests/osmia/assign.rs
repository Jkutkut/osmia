use super::*;

macro_tests!(
	test,
	(
		assign01,
		Some("{{ v = 1 }}{{ v }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Int(1)
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into())
			])).into()
		].into())),
		None
		// r#"{}"#,
		// "1"
		// r#"{"v": 2}"#,
		// "1"
	),
	(
		assign_string,
		Some("{{ v = \"foo\" }}{{ v }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::new_str("foo"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::new_str("foo")
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into())
			])).into()
		].into())),
		None
		// r#"{}"#,
		// "foo"
	),
	(
		assign_int,
		Some("{{ v = 1 }}{{ v }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Int(1)
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into())
			])).into()
		].into())),
		None
		// r#"{}"#,
		// "1"
	),
	(
		assign_float,
		Some("{{  v = 1.1 }}{{ v }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::new_number("1.1"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Float(1.1)
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into())
			])).into()
		].into())),
		None
		// r#"{}"#,
		// "1.1"
	),
	(
		assign_bool,
		Some("{{ v = true }}{{ v }} -- {{ v = false }}{{ v }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::Bool(true),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::new_raw(" -- "),
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::Bool(false),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Bool(true)
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into())
			])).into(),
			Stmt::new_raw(" -- "),
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Bool(false)
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into())
			])).into()
		].into())),
		None
		// r#"{}"#,
		// "true -- false"
	),
	(
		assign_null,
		Some("{{ v = null }}{{ v }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::Null,
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Null,
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into())
			])).into()
		].into())),
		None
		// r#"{}"#,
		// "null"
	),
	(
		assign_array_item,
		Some("{{ v[2] = 2 }}{{ v[0] }}{{ v[1] }}{{ v[2] }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("2"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::new_number("2"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("1"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("2"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into()),
					JsonTreeKeyExpression::Expr(Expr::Int(2))
				]),
				Expr::Int(2),
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::Expr(Expr::Int(0))
			])).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::Expr(Expr::Int(1))
			])).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::Expr(Expr::Int(2))
			])).into()
		].into())),
		None
		// r#"{"v": [1, 2,  3]}"#,
		// "122"
	),
	(
		assign_override01,
		Some("{{ v = 1 }}{{v}}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::new_alpha("v"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Int(1)
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into())
			])).into()
		].into())),
		None
		// r#"{"v": {}}"#,
		// "1"
		// r#"{"v": [123, 2,  3]}"#,
		// "1"
	),
	(
		assign_variable01,
		Some("{{ foo = bar }}{{foo}}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::new_alpha("bar"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::new_alpha("foo"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("foo".into())
				]),
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("bar".into())
				]).into()
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("foo".into())
			])).into()
		].into())),
		None
		// r#"{"bar": 2}"#,
		// "2"
	),
	(
		assign_variable02,
		Some("{{ foo = bar * foo }}{{foo}}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::new_alpha("bar"),
			Token::Whitespace,
			Token::Mult,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::new_alpha("foo"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("foo".into())
				]),
				Binary::new(
					Variable::from_vec(vec![
						JsonTreeKeyExpression::JsonTreeKey("bar".into())
					]).into(),
					BinaryOp::Mult,
					Variable::from_vec(vec![
						JsonTreeKeyExpression::JsonTreeKey("foo".into())
					]).into()
				).into()
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("foo".into())
			])).into()
		].into())),
		None
		// r#"{"bar": 2, "foo": 2}"#,
		// "4"
	),
	(
		assign_array01,
		Some("{{ v = [1,  2,  3] }}{{ v[0] }}{{ v[1] }}{{ v[2] }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
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
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("1"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("2"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Array(vec![
					Expr::Int(1), Expr::Int(2), Expr::Int(3)
				].into())
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::Expr(Expr::Int(0))
			])).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::Expr(Expr::Int(1))
			])).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::Expr(Expr::Int(2))
			])).into()
		].into())),
		None
		// "{}",
		// "123"
	),
	(
		assign_array02,
		Some(r#"{{ v = [{"name": "foo"},  "this",  [3]]}}{{ v[0].name }} -- {{ v[1] }} -- {{ v[2][0] }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
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
			Token::new_str("this"),
			Token::Comma,
			Token::Whitespace,
			Token::ArrayStart,
			Token::new_number("3"),
			Token::ArrayEnd,
			Token::ArrayEnd,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Dot,
			Token::new_alpha("name"),
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
			Token::new_raw(" -- "),
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::ArrayStart,
			Token::new_number("2"),
			Token::ArrayEnd,
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Array(vec![
					Expr::Object(vec![
						(Expr::new_str("name"), Expr::new_str("foo"))
					].into()),
					Expr::new_str("this"),
					Expr::Array(vec![Expr::Int(3)].into())
				].into())
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::Expr(Expr::Int(0)),
				JsonTreeKeyExpression::JsonTreeKey("name".into())
			])).into(),
			Stmt::new_raw(" -- "),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::Expr(Expr::Int(1))
			])).into(),
			Stmt::new_raw(" -- "),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::Expr(Expr::Int(2)),
				JsonTreeKeyExpression::Expr(Expr::Int(0))
			])).into()
		].into())),
		None
		// "{}",
		// "foo -- this -- 3"
	),
	(
		assign_object01,
		Some(r#"{{v = {"foo": 1,  "bar": 2} }}{{ v.foo }} -- {{ v.bar }}"#),
		Some(vec![
			Token::StmtStart,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::ObjectStart,
			Token::new_str("foo"),
			Token::Colon,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Comma,
			Token::Whitespace,
			Token::new_str("bar"),
			Token::Colon,
			Token::Whitespace,
			Token::new_number("2"),
			Token::ObjectEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Dot,
			Token::new_alpha("foo"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::new_raw(" -- "),
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Dot,
			Token::new_alpha("bar"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Object(vec![
					(Expr::new_str("foo"), Expr::Int(1)),
					(Expr::new_str("bar"), Expr::Int(2))
				].into())
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::JsonTreeKey("foo".into())
			])).into(),
			Stmt::new_raw(" -- "),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::JsonTreeKey("bar".into())
			])).into()
		].into())),
		None
		// "{}",
		// "1 -- 2"
	),
	(
		assign_object02,
		Some(r#"{{ v = {"foo": {"bar": 1},  "bar": [2]} }}{{ v.foo.bar }} -- {{ v.bar[0] }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::ObjectStart,
			Token::new_str("foo"),
			Token::Colon,
			Token::Whitespace,
			Token::ObjectStart,
			Token::new_str("bar"),
			Token::Colon,
			Token::Whitespace,
			Token::new_number("1"),
			Token::ObjectEnd,
			Token::Comma,
			Token::Whitespace,
			Token::new_str("bar"),
			Token::Colon,
			Token::Whitespace,
			Token::ArrayStart,
			Token::new_number("2"),
			Token::ArrayEnd,
			Token::ObjectEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Dot,
			Token::new_alpha("foo"),
			Token::Dot,
			Token::new_alpha("bar"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::new_raw(" -- "),
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("v"),
			Token::Dot,
			Token::new_alpha("bar"),
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("v".into())
				]),
				Expr::Object(vec![
					(Expr::new_str("foo"), Expr::Object(vec![
						(Expr::new_str("bar"), Expr::Int(1))
					].into())),
					(Expr::new_str("bar"), Expr::Array(vec![Expr::Int(2)].into()))
				].into())
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::JsonTreeKey("foo".into()),
				JsonTreeKeyExpression::JsonTreeKey("bar".into())
			])).into(),
			Stmt::new_raw(" -- "),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("v".into()),
				JsonTreeKeyExpression::JsonTreeKey("bar".into()),
				JsonTreeKeyExpression::Expr(Expr::Int(0))
			])).into()
		].into())),
		None
		// "{}",
		// "1 -- 2"
	),
	(
		assign_object03,
		Some(r#"{{ obj = {"user": {"name": "Marvin"} } }}{{ obj.user.name = "R2D2" }}{{ obj.user.name }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("obj"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::ObjectStart,
			Token::new_str("user"),
			Token::Colon,
			Token::Whitespace,
			Token::ObjectStart,
			Token::new_str("name"),
			Token::Colon,
			Token::Whitespace,
			Token::new_str("Marvin"),
			Token::ObjectEnd,
			Token::Whitespace,
			Token::ObjectEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("obj"),
			Token::Dot,
			Token::new_alpha("user"),
			Token::Dot,
			Token::new_alpha("name"),
			Token::Whitespace,
			Token::Assign,
			Token::Whitespace,
			Token::new_str("R2D2"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("obj"),
			Token::Dot,
			Token::new_alpha("user"),
			Token::Dot,
			Token::new_alpha("name"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("obj".into())
				]),
				Expr::Object(vec![
					(Expr::new_str("user"), Expr::Object(vec![
						(Expr::new_str("name"), Expr::new_str("Marvin"))
					].into()))
				].into())
			).into(),
			Stmt::new_assign(
				Variable::from_vec(vec![
					JsonTreeKeyExpression::JsonTreeKey("obj".into()),
					JsonTreeKeyExpression::JsonTreeKey("user".into()),
					JsonTreeKeyExpression::JsonTreeKey("name".into())
				]),
				Expr::new_str("R2D2")
			).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpression::JsonTreeKey("obj".into()),
				JsonTreeKeyExpression::JsonTreeKey("user".into()),
				JsonTreeKeyExpression::JsonTreeKey("name".into())
			])).into()
		].into())),
		None
		// "{}",
		// "R2D2"
	)
);
