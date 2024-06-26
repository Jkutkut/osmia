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
			Token::Value("a".to_string()),
			Token::In,
			Token::Value("lst".to_string()),
			Token::DelimiterEnd,
			Token::Raw("This line is constant".to_string()),
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd
		],
		Stmt::ForEach(ForEach::new(
			Variable::from_str("a").unwrap(),
			ListOrVariable::Variable(Variable::from_str("lst").unwrap()),
			Stmt::Raw("This line is constant".to_string())
		))
	),
	(
		nested, // {{for arr in matrix}}{{for cell in arr}}{{print cell}}{{end}}{{end}}
		vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("arr".to_string()),
			Token::In,
			Token::Value("matrix".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::For,
			Token::Value("cell".to_string()),
			Token::In,
			Token::Value("arr".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("cell".to_string()),
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
			Token::Value("a".to_string()),
			Token::In,
			Token::ArrayStart,
			Token::Value("1".to_string()),
			Token::Comma,
			Token::Value("2".to_string()),
			Token::Comma,
			Token::Value("3".to_string()),
			Token::ArrayEnd,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("a".to_string()),
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
			Token::Value("a".to_string()),
			Token::In,
			Token::Value("lst".to_string()),
			Token::DelimiterEnd
		]
	),
	(
		fail02,
		vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("a".to_string()),
			Token::In
		]
	),
	(
		fail03,
		vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("a".to_string())
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
