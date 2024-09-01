use super::*;

macro_tests! {
	test,
	(
		empty,
		Some(""),
		Some(vec![
			Token::Eof
		]),
		Some(Stmt::Block(Block::new())),
		None // ""
	),
	(
		just_text,
		Some("Hello, world!"),
		Some(vec![
			Token::new_raw("Hello, world!"),
			Token::Eof
		]),
		Some(Stmt::Raw("Hello, world!".to_string())),
		None // "Hello, world!"
	),
	(
		basic01,
		Some("{{true}}"),
		Some(vec![
			Token::StmtStart,
			Token::Bool(true),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Bool(true).into()),
		None // "true"
	),
	(
		basic02,
		Some("{{false}}"),
		Some(vec![
			Token::StmtStart,
			Token::Bool(false),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Bool(false).into()),
		None // "false"
	),
	(
		basic03,
		Some("{{null}}"),
		Some(vec![
			Token::StmtStart,
			Token::Null,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Null.into()),
		None // "null"
	),
	(
		basic04,
		Some("{{42}}"),
		Some(vec![
			Token::StmtStart,
			Token::new_number("42"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Int(42).into()),
		None // "42"
	),
	(
		basic05,
		Some("{{3.14}}"),
		Some(vec![
			Token::StmtStart,
			Token::new_number("3.14"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Float(3.14).into()),
		None // "3.14"
	),
	(
		basic06,
		Some(r#"{{"Hello, world!"}}"#),
		Some(vec![
			Token::StmtStart,
			Token::new_str("Hello, world!"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::new_str("Hello, world!").into()),
		None // "Hello, world!"
	),
	(
		basic07,
		Some(r#"{{""}}"#),
		Some(vec![
			Token::StmtStart,
			Token::new_str(""),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::new_str("").into()),
		None // ""
	),
	(
		basic08,
		Some("{{\"\\n\"}}{{\"\n\"}}"),
		Some(vec![
			Token::StmtStart,
			Token::new_str("\\n"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::NewLine,
			Token::new_str("\n"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::new_str("\\n").into(),
			Expr::new_str("\n").into(),
		].into())),
		None // "\\n\n"
	),
	(
		basic09,
		Some("{{\"\\r\"}}{{\"\r\"}}"),
		Some(vec![
			Token::StmtStart,
			Token::new_str("\\r"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::new_str("\r"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::new_str("\\r").into(),
			Expr::new_str("\r").into(),
		].into())),
		None // "\\r\r"
	),
	(
		basic10,
		Some("{{\"\\t\"}}{{\"\t\"}}"),
		Some(vec![
			Token::StmtStart,
			Token::new_str("\\t"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::new_str("\t"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::new_str("\\t").into(),
			Expr::new_str("\t").into(),
		].into())),
		None // "\\t\t"
	),
	(
		basic11,
		Some("{{true}} {{false}}"),
		Some(vec![
			Token::StmtStart,
			Token::Bool(true),
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::Bool(false),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::Bool(true).into(),
			Stmt::new_raw(" "),
			Expr::Bool(false).into(),
		].into())),
		None // "true false"
	),
	(
		basic12,
		Some("{{true}} {{false}} {{null}}"),
		Some(vec![
			Token::StmtStart,
			Token::Bool(true),
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::Bool(false),
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::Null,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::Bool(true).into(),
			Stmt::new_raw(" "),
			Expr::Bool(false).into(),
			Stmt::new_raw(" "),
			Expr::Null.into(),
		].into())),
		None // "true false null"
	),
	(
		basic13,
		Some("{{true}} {{false}} {{42}}"),
		Some(vec![
			Token::StmtStart,
			Token::Bool(true),
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::Bool(false),
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::new_number("42"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::Bool(true).into(),
			Stmt::new_raw(" "),
			Expr::Bool(false).into(),
			Stmt::new_raw(" "),
			Expr::Int(42).into(),
		].into())),
		None // "true false 42"
	),
	(
		basic14,
		Some("{{true}} {{false}} {{3.14}}"),
		Some(vec![
			Token::StmtStart,
			Token::Bool(true),
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::Bool(false),
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::new_number("3.14"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::Bool(true).into(),
			Stmt::new_raw(" "),
			Expr::Bool(false).into(),
			Stmt::new_raw(" "),
			Expr::Float(3.14).into(),
		].into())),
		None // "true false 3.14"
	),
	(
		basic15,
		Some(r#"{{"Hello, world!"}} {{42}}"#),
		Some(vec![
			Token::StmtStart,
			Token::new_str("Hello, world!"),
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::new_number("42"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::new_str("Hello, world!").into(),
			Stmt::new_raw(" "),
			Expr::Int(42).into(),
		].into())),
		None // "Hello, world! 42"
	),
	(
		precedence,
		Some("{{ 1 + 2 * 3 / (4 + 5) == 1 + (2 * 3) % 9 == true }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_number("2"),
			Token::Whitespace,
			Token::Mult,
			Token::Whitespace,
			Token::new_number("3"),
			Token::Whitespace,
			Token::Div,
			Token::Whitespace,
			Token::ParentStart,
			Token::new_number("4"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_number("5"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::Equal,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::ParentStart,
			Token::new_number("2"),
			Token::Whitespace,
			Token::Mult,
			Token::Whitespace,
			Token::new_number("3"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::Mod,
			Token::Whitespace,
			Token::new_number("9"),
			Token::Whitespace,
			Token::Equal,
			Token::Whitespace,
			Token::Bool(true),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(new_binary(
			new_binary(
				new_binary(
					Expr::Int(1).into(),
					Token::Plus,
					new_binary(
						new_binary(
							Expr::Int(2).into(),
							Token::Mult,
							Expr::Int(3).into()
						),
						Token::Div,
						Grouping::new(new_binary(
							Expr::Int(4).into(),
							Token::Plus,
							Expr::Int(5).into()
						)).into()
					).into()
				).into(),
				Token::Equal,
				new_binary(
					Expr::Int(1).into(),
					Token::Plus,
					new_binary(
						Grouping::new(new_binary(
							Expr::Int(2).into(),
							Token::Mult,
							Expr::Int(3).into()
						)).into(),
						Token::Mod,
						Expr::Int(9).into()
					).into()
				).into()
			).into(),
			Token::Equal,
			Expr::Bool(true).into()
		).into()),
		None
	),
	(
		unary_operators01,
		Some("{{ !!true == !false == (-(-1) == 1) != false }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Not,
			Token::Not,
			Token::Bool(true),
			Token::Whitespace,
			Token::Equal,
			Token::Whitespace,
			Token::Not,
			Token::Bool(false),
			Token::Whitespace,
			Token::Equal,
			Token::Whitespace,
			Token::ParentStart,
			Token::Minus,
			Token::ParentStart,
			Token::Minus,
			Token::new_number("1"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::Equal,
			Token::Whitespace,
			Token::new_number("1"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::NotEqual,
			Token::Whitespace,
			Token::Bool(false),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(new_binary(
			new_binary(
				new_binary(
					new_unary(
						Token::Not,
						new_unary(
							Token::Not,
							Expr::Bool(true).into()
						).into()
					).into(),
					Token::Equal,
					new_unary(
						Token::Not,
						Expr::Bool(false).into()
					).into()
				).into(),
				Token::Equal,
				Grouping::new(new_binary(
					new_unary(
						Token::Minus,
						Grouping::new(new_unary(
							Token::Minus,
							Expr::Int(1).into()
						)).into()
					).into(),
					Token::Equal,
					Expr::Int(1).into()
				).into()).into()
			).into(),
			Token::NotEqual,
			Expr::Bool(false).into()
		).into()),
		None
	),
	(
		unary_operators02,
		Some("{{ ---1 == -(-(-(1))) == -1 }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Minus,
			Token::Minus,
			Token::Minus,
			Token::new_number("1"),
			Token::Whitespace,
			Token::Equal,
			Token::Whitespace,
			Token::Minus,
			Token::ParentStart,
			Token::Minus,
			Token::ParentStart,
			Token::Minus,
			Token::ParentStart,
			Token::new_number("1"),
			Token::ParentEnd,
			Token::ParentEnd,
			Token::ParentEnd,
			Token::Whitespace,
			Token::Equal,
			Token::Whitespace,
			Token::Minus,
			Token::new_number("1"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(new_binary(
			new_binary(
				new_unary(
					Token::Minus,
					new_unary(
						Token::Minus,
						new_unary(Token::Minus, Expr::Int(1).into()).into()
					).into()
				).into(),
				Token::Equal,
				new_unary(
					Token::Minus,
					Grouping::new(new_unary(
						Token::Minus,
						Grouping::new(new_unary(
							Token::Minus,
							Grouping::new(Expr::Int(1).into()).into()
						)).into()
					)).into()
				).into()
			).into(),
			Token::Equal,
			new_unary(Token::Minus, Expr::Int(1).into()).into()
		).into()),
		None
	),
	(
		grouping,
		Some("{{ (1 + 2) * 3 == 9 }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::ParentStart,
			Token::new_number("1"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_number("2"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::Mult,
			Token::Whitespace,
			Token::new_number("3"),
			Token::Whitespace,
			Token::Equal,
			Token::Whitespace,
			Token::new_number("9"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(new_binary(
			new_binary(
				Grouping::new(new_binary(
					Expr::Int(1).into(),
					Token::Plus,
					Expr::Int(2).into()
				)).into(),
				Token::Mult,
				Expr::Int(3).into()
			).into(),
			Token::Equal,
			Expr::Int(9).into()
		).into()),
		None
	),
	(
		json01,
		Some("{{ [1, 2, 3] }}"),
		Some(vec![
			Token::StmtStart,
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
			Token::Eof
		]),
		Some(Expr::Array(vec![
			Expr::Int(1).into(),
			Expr::Int(2).into(),
			Expr::Int(3).into()
		].into()).into()),
		None // "[1, 2, 3]"
	),
	(
		json02,
		Some(r#"{{ {"a": 1, "b": 2, "c": 3} }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::ObjectStart,
			Token::new_str("a"),
			Token::Colon,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Comma,
			Token::Whitespace,
			Token::new_str("b"),
			Token::Colon,
			Token::Whitespace,
			Token::new_number("2"),
			Token::Comma,
			Token::Whitespace,
			Token::new_str("c"),
			Token::Colon,
			Token::Whitespace,
			Token::new_number("3"),
			Token::ObjectEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Object(vec![
			(Expr::new_str("a"), Expr::Int(1)),
			(Expr::new_str("b"), Expr::Int(2)),
			(Expr::new_str("c"), Expr::Int(3)),
		].into()).into()),
		None // "{a: 1, b: 2, c: 3}"
	),
	(
		json03,
		Some(r#"{{ [ 1, 2, {"foo": [3, 4]} ] }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::ArrayStart,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("2"),
			Token::Comma,
			Token::Whitespace,
			Token::ObjectStart,
			Token::new_str("foo"),
			Token::Colon,
			Token::Whitespace,
			Token::ArrayStart,
			Token::new_number("3"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("4"),
			Token::ArrayEnd,
			Token::ObjectEnd,
			Token::Whitespace,
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Array(vec![
			Expr::Int(1).into(),
			Expr::Int(2).into(),
			Expr::Object(vec![(
				Expr::new_str("foo"),
				Expr::Array(vec![
					Expr::Int(3), Expr::Int(4)
				].into())
			)].into())
		].into()).into()),
		None // "[1, 2, {foo: [3, 4]}]"
	),
	(
		json04,
		Some(r#"{{ {"bar": [4, 5, 6], "foo": [1, 2, 3]} }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::ObjectStart,
			Token::new_str("bar"),
			Token::Colon,
			Token::Whitespace,
			Token::ArrayStart,
			Token::new_number("4"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("5"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("6"),
			Token::ArrayEnd,
			Token::Comma,
			Token::Whitespace,
			Token::new_str("foo"),
			Token::Colon,
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
			Token::ObjectEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Expr::Object(vec![
			(
				Expr::new_str("bar"),
				Expr::Array(vec![
					Expr::Int(4), Expr::Int(5), Expr::Int(6)
				].into())
			),
			(
				Expr::new_str("foo"),
				Expr::Array(vec![
					Expr::Int(1), Expr::Int(2), Expr::Int(3)
				].into())
			)
		].into()).into()),
		None // "{bar: [4, 5, 6], foo: [1, 2, 3]}"
	),
	(
		json05,
		Some(r#"{{ {} }} {{ [] }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::ObjectStart,
			Token::ObjectEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::Whitespace,
			Token::ArrayStart,
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::Object(vec![].into()).into(),
			Stmt::new_raw(" "),
			Expr::Array(vec![].into()).into()
		].into()).into()),
		None // "{} []"
	),
	(
		json06,
		Some(r#"{{ { } }} {{ [ ] }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::ObjectStart,
			Token::Whitespace,
			Token::ObjectEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::Whitespace,
			Token::ArrayStart,
			Token::Whitespace,
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Expr::Object(vec![].into()).into(),
			Stmt::new_raw(" "),
			Expr::Array(vec![].into()).into()
		].into()).into()),
		None // "{} []"
	),
}
