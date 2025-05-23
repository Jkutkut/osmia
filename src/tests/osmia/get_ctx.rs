use super::*;

macro_tests!(
	test,
	(
		variable01,
		Some("{{ foo }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Variable(Variable::from_vec(vec![
			JsonTreeKeyExpr::JsonTreeKey("foo".into()),
		])).into()),
		None
		// r#"{"foo": "bar"}"#,
		// "bar"
		// "{}"
		// "null"),
	),
	(
		variable02,
		Some("{{ foo.bar }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::Dot,
			Token::new_alpha("bar"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Variable(Variable::from_vec(vec![
			JsonTreeKeyExpr::JsonTreeKey("foo".into()),
			JsonTreeKeyExpr::JsonTreeKey("bar".into()),
		])).into()),
		None
		// r#"{"foo": {"bar": "baz"}}"#,
		// "baz"
		// r#"{"foo": {}}"#,
		// "null"
	),
	(
		variable03,
		Some("{{ foo._bar.baz }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::Dot,
			Token::new_alpha("_bar"),
			Token::Dot,
			Token::new_alpha("baz"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Variable(Variable::from_vec(vec![
			JsonTreeKeyExpr::JsonTreeKey("foo".into()),
			JsonTreeKeyExpr::JsonTreeKey("_bar".into()),
			JsonTreeKeyExpr::JsonTreeKey("baz".into()),
		])).into()),
		None
		// r#"{"foo": {"bar": {"baz": "qux"}}}"#,
		// "qux"
	),
	(
		variable04,
		Some("{{ arr[0] }} {{ arr[1] }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("arr"),
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("arr"),
			Token::ArrayStart,
			Token::new_number("1"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("arr".into()),
				JsonTreeKeyExpr::Expr(Expr::Int(0)),
			])).into(),
			Stmt::new_raw(" "),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("arr".into()),
				JsonTreeKeyExpr::Expr(Expr::Int(1)),
			])).into()
		].into())),
		None
		// r#"{"arr": ["foo", "bar"]}"#,
		// "foo bar"
	),
	(
		variable05,
		Some("{{ arr[0].name }} {{ arr[1].surname }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("arr"),
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Dot,
			Token::new_alpha("name"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("arr"),
			Token::ArrayStart,
			Token::new_number("1"),
			Token::ArrayEnd,
			Token::Dot,
			Token::new_alpha("surname"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("arr".into()),
				JsonTreeKeyExpr::Expr(Expr::Int(0)),
				JsonTreeKeyExpr::JsonTreeKey("name".into()),
			])).into(),
			Stmt::new_raw(" "),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("arr".into()),
				JsonTreeKeyExpr::Expr(Expr::Int(1)),
				JsonTreeKeyExpr::JsonTreeKey("surname".into()),
			])).into(),
		].into())),
		None
		// r#"{"arr": [{"name": "foo"}, {"name": "bar", "surname": "baz"}]}"#,
		// "foo baz"
	)
);

macro_tests!(
	test,
	(
		advanced_json_control01,
		Some(r#"{{ foo[v] }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ArrayStart,
			Token::new_alpha("v"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Variable(Variable::from_vec(vec![
			JsonTreeKeyExpr::JsonTreeKey("foo".into()),
			JsonTreeKeyExpr::Expr(Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into())
			]))),
		])).into()),
		None
		// r#"{"foo": [1, 2, 3], "v": 1}"#,
		// "2"
	),
	(
		advanced_json_control02,
		Some(r#"{{ foo["bar"] }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ArrayStart,
			Token::new_str("bar"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Variable(Variable::from_vec(vec![
			JsonTreeKeyExpr::JsonTreeKey("foo".into()),
			JsonTreeKeyExpr::Expr(Expr::new_str("bar")),
		])).into()),
		None
		// r#"{"foo": {"bar": "baz"}, "v": 1}"#,
		// "baz"
	),
	(
		advanced_json_control03,
		Some(r#"{{ foo['bar'] }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ArrayStart,
			Token::new_str("bar"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Variable(Variable::from_vec(vec![
			JsonTreeKeyExpr::JsonTreeKey("foo".into()),
			JsonTreeKeyExpr::Expr(Expr::new_str("bar")),
		])).into()),
		None
		// r#"{"foo": {"bar": "baz"}, "v": 1}"#,
		// "baz"
	),
	(
		advanced_json_control04,
		Some(r#"{{ foo[v] }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ArrayStart,
			Token::new_alpha("v"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Variable(Variable::from_vec(vec![
			JsonTreeKeyExpr::JsonTreeKey("foo".into()),
			JsonTreeKeyExpr::Expr(Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("v".into()),
			]))),
		])).into()),
		None
		// r#"{"foo": {"bar": "baz"}, "v": "bar"}"#,
		// "baz"
	),
	(
		advanced_variable_name01,
		Some(r#"{{ foo123_bar }}{{_hidden123}}{{_1}}{{z_}}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo123_bar"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::StmtStart,
			Token::new_alpha("_hidden123"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::new_alpha("_1"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::new_alpha("z_"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("foo123_bar".into()),
			])).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("_hidden123".into()),
			])).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("_1".into()),
			])).into(),
			Expr::Variable(Variable::from_vec(vec![
				JsonTreeKeyExpr::JsonTreeKey("z_".into()),
			])).into(),
		].into())),
		None
	),
	(
		mixed_variable_name01,
		Some(r#"{{ foo[0].bar[1][2].baz.qux }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Dot,
			Token::new_alpha("bar"),
			Token::ArrayStart,
			Token::new_number("1"),
			Token::ArrayEnd,
			Token::ArrayStart,
			Token::new_number("2"),
			Token::ArrayEnd,
			Token::Dot,
			Token::new_alpha("baz"),
			Token::Dot,
			Token::new_alpha("qux"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Variable(Variable::from_vec(vec![
			JsonTreeKeyExpr::JsonTreeKey("foo".into()),
			JsonTreeKeyExpr::Expr(Expr::Int(0)),
			JsonTreeKeyExpr::JsonTreeKey("bar".into()),
			JsonTreeKeyExpr::Expr(Expr::Int(1)),
			JsonTreeKeyExpr::Expr(Expr::Int(2)),
			JsonTreeKeyExpr::JsonTreeKey("baz".into()),
			JsonTreeKeyExpr::JsonTreeKey("qux".into()),
		])).into()),
		None
	)
);
