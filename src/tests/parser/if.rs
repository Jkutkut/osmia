use crate::lexer::Token;
use crate::model::{
	Expression, Literal, Binary, Variable, JsonExpression,
	Stmt, ConditionalBlock, If, Block
};
use super::{test_parser, should_fail};
use crate::macro_tests;

macro_tests!(
	test_parser,
	( // {{if condition == "if"}}{{print "condition is if"}}{{fi}}
		basic_test01,
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition".to_string()),
			Token::Equal,
			Token::Value(r#""if""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is if""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Fi,
			Token::DelimiterEnd
		],
		Stmt::If(If::new(
			ConditionalBlock::new(
				Expression::Binary(Binary::new(
					Expression::Variable(Variable::from_str("condition").unwrap()),
					Token::Equal,
					Expression::Literal(Literal::from_str(r#""if""#).unwrap())
				).unwrap()),
				Stmt::Print(JsonExpression::Expression(Expression::Literal(
					Literal::from_str(r#""condition is if""#).unwrap())
				))
			),
			None,
			None
		))
	),
	//	{{if condition == "if"}}
	//		{{print "condition is if"}}
	//	{{else}}
	//		{{print "condition is else"}}
	//	{{fi}}
	(
		basic_test02,
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition".to_string()),
			Token::Equal,
			Token::Value(r#""if""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is if""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Else,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is else""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Fi,
			Token::DelimiterEnd
		],
		Stmt::If(If::new(
			ConditionalBlock::new(
				Expression::Binary(Binary::new(
					Expression::Variable(Variable::from_str("condition").unwrap()),
					Token::Equal,
					Expression::Literal(Literal::from_str(r#""if""#).unwrap())
				).unwrap()),
				Stmt::Print(JsonExpression::Expression(Expression::Literal(
					Literal::from_str(r#""condition is if""#).unwrap())
				))
			),
			None,
			Some(Stmt::Print(JsonExpression::Expression(Expression::Literal(
				Literal::from_str(r#""condition is else""#).unwrap())
			)))
		))
	),
	//	{{if condition == "if"}}
	//		{{print "condition is if"}}
	//	{{elseif condition == "elseif01"}}
	//		{{print "condition is elseif01"}}
	//	{{elseif condition == "elseif02"}}
	//		{{print "condition is elseif02"}}
	//	{{else}}
	//		{{print "condition is else"}}
	//	{{fi}}
	(
		basic_test03,
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition".to_string()),
			Token::Equal,
			Token::Value(r#""if""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is if""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::ElseIf,
			Token::Value("condition".to_string()),
			Token::Equal,
			Token::Value(r#""elseif01""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is elseif01""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::ElseIf,
			Token::Value("condition".to_string()),
			Token::Equal,
			Token::Value(r#""elseif02""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is elseif02""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Else,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is else""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Fi,
			Token::DelimiterEnd
		],
		Stmt::If(If::new(
			ConditionalBlock::new(
				Expression::Binary(Binary::new(
					Expression::Variable(Variable::from_str("condition").unwrap()),
					Token::Equal,
					Expression::Literal(Literal::from_str(r#""if""#).unwrap())
				).unwrap()),
				Stmt::Print(JsonExpression::Expression(Expression::Literal(
					Literal::from_str(r#""condition is if""#).unwrap())
				))
			),
			Some(vec![
				ConditionalBlock::new(
					Expression::Binary(Binary::new(
						Expression::Variable(Variable::from_str("condition").unwrap()),
						Token::Equal,
						Expression::Literal(Literal::from_str(r#""elseif01""#).unwrap())
					).unwrap()),
					Stmt::Print(JsonExpression::Expression(Expression::Literal(
						Literal::from_str(r#""condition is elseif01""#).unwrap())
					))
				),
				ConditionalBlock::new(
					Expression::Binary(Binary::new(
						Expression::Variable(Variable::from_str("condition").unwrap()),
						Token::Equal,
						Expression::Literal(Literal::from_str(r#""elseif02""#).unwrap())
					).unwrap()),
					Stmt::Print(JsonExpression::Expression(Expression::Literal(
						Literal::from_str(r#""condition is elseif02""#).unwrap())
					))
				),
			]),
			Some(Stmt::Print(JsonExpression::Expression(Expression::Literal(
				Literal::from_str(r#""condition is else""#).unwrap())
			)))
		))
	),
	// {{if condition == "if"}}{{fi}}
	(
		empty_if_block,
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition".to_string()),
			Token::Equal,
			Token::Value(r#""if""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Fi,
			Token::DelimiterEnd
		],
		Stmt::If(If::new(
			ConditionalBlock::new(
				Expression::Binary(Binary::new(
					Expression::Variable(Variable::from_str("condition").unwrap()),
					Token::Equal,
					Expression::Literal(Literal::from_str(r#""if""#).unwrap())
				).unwrap()),
				Stmt::Block(Block::new(vec![]))
			),
			None,
			None
		))
	)
);

macro_tests!(
	should_fail,
	(
		invalid01,
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition".to_string()),
			Token::Equal,
			Token::Value(r#""if""#.to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Fi
		]
	),
	(
		invalid02,
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition".to_string()),
			Token::Equal,
			Token::Value(r#""if""#.to_string()),
			Token::DelimiterEnd
		]
	),
	(
		invalid03,
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition".to_string()),
			Token::Equal,
			Token::Value(r#""if""#.to_string()),
		]
	),
	(
		invalid04,
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition".to_string()),
			Token::Equal
		]
	),
	(
		invalid05,
		vec![
			Token::DelimiterStart,
			Token::If
		]
	),
	(
		invalid06,
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::DelimiterEnd
		]
	)
);
