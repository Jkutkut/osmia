use crate::lexer::Token;
use crate::model::{
	Variable, Expression, Literal,
	ListOrVariable, JsonExpression,
	Stmt, ForEach
};
use super::{test_parser, should_fail};
use crate::macro_tests;

macro_tests!(
	test_parser,
	(
		test_parser01, // {{for a in lst}}This line is constant{{end}}
		vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("a"),
			Token::In,
			Token::Value("lst"),
			Token::DelimiterEnd,
			Token::Raw("This line is constant"),
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd
		],
		Stmt::ForEach(ForEach::new(
			Variable::from_str("a").unwrap(),
			ListOrVariable::Variable(Variable::from_str("lst").unwrap()),
			Stmt::Raw("This line is constant")
		))
	),
	(
		nested, // {{for arr in matrix}}{{for cell in arr}}{{print cell}}{{end}}{{end}}
		vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("arr"),
			Token::In,
			Token::Value("matrix"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::For,
			Token::Value("cell"),
			Token::In,
			Token::Value("arr"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("cell"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd
		],
		Stmt::ForEach(ForEach::new(
			Variable::from_str("arr").unwrap(),
			ListOrVariable::Variable(Variable::from_str("matrix").unwrap()),
			Stmt::ForEach(ForEach::new(
				Variable::from_str("cell").unwrap(),
				ListOrVariable::Variable(Variable::from_str("arr").unwrap()),
				Stmt::Print(
					Expression::Variable(
						Variable::from_str("cell").unwrap()
					)
				)
			))
		))
	),
	(
		list, // {{for a in [1, 2, 3]}}{{print a}}{{end}}
		vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("a"),
			Token::In,
			Token::ArrayStart,
			Token::Value("1"),
			Token::Comma,
			Token::Value("2"),
			Token::Comma,
			Token::Value("3"),
			Token::ArrayEnd,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("a"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd
		],
		Stmt::ForEach(ForEach::new(
			Variable::from_str("a").unwrap(),
			ListOrVariable::List(JsonExpression::Array(vec![
				JsonExpression::Expression(Expression::Literal(Literal::Int(1))),
				JsonExpression::Expression(Expression::Literal(Literal::Int(2))),
				JsonExpression::Expression(Expression::Literal(Literal::Int(3)))
			])),
			Stmt::Print(Expression::Variable(Variable::from_str("a").unwrap()))
		))
	)
);

// Fail tests
macro_tests!(
	should_fail,
	(
		should_fail01,
			vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("a"),
			Token::In,
			Token::Value("lst"),
			Token::DelimiterEnd
		]
	),
	(
		fail02,
		vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("a"),
			Token::In
		]
	),
	(
		fail03,
		vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("a")
		]
	),
	(
		fail04,
		vec![
			Token::DelimiterStart,
			Token::For
		]
	)
);
