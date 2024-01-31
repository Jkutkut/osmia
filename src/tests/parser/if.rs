use crate::Token;
use crate::syntax_tree::model::{
	Expression, Literal, Binary, Variable,
	Stmt, ConditionalBlock, If, Block
};
use super::{test_parser};

#[test]
fn basic_test01() {
	test_parser( // {{if condition == "if"}}{{print "condition is if"}}{{fi}}
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition"),
			Token::Equal,
			Token::Value(r#""if""#),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is if""#),
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
				Stmt::Print(Expression::Literal(
					Literal::from_str(r#""condition is if""#).unwrap())
				)
			),
			None,
			None
		))
	);
}

#[test]
fn basic_test02() {
	//	{{if condition == "if"}}
	//		{{print "condition is if"}}
	//	{{else}}
	//		{{print "condition is else"}}
	//	{{fi}}
	test_parser(
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition"),
			Token::Equal,
			Token::Value(r#""if""#),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is if""#),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Else,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is else""#),
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
				Stmt::Print(Expression::Literal(
					Literal::from_str(r#""condition is if""#).unwrap())
				)
			),
			None,
			Some(Stmt::Print(Expression::Literal(
				Literal::from_str(r#""condition is else""#).unwrap())
			))
		))
	);
}

#[test]
fn basic_test03() {
	//	{{if condition == "if"}}
	//		{{print "condition is if"}}
	//	{{elseif condition == "elseif01"}}
	//		{{print "condition is elseif01"}}
	//	{{elseif condition == "elseif02"}}
	//		{{print "condition is elseif02"}}
	//	{{else}}
	//		{{print "condition is else"}}
	//	{{fi}}
	test_parser(
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition"),
			Token::Equal,
			Token::Value(r#""if""#),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is if""#),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::ElseIf,
			Token::Value("condition"),
			Token::Equal,
			Token::Value(r#""elseif01""#),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is elseif01""#),
			Token::DelimiterEnd,

			Token::DelimiterStart,
			Token::ElseIf,
			Token::Value("condition"),
			Token::Equal,
			Token::Value(r#""elseif02""#),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is elseif02""#),
			Token::DelimiterEnd,

			Token::DelimiterStart,
			Token::Else,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value(r#""condition is else""#),
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
				Stmt::Print(Expression::Literal(
					Literal::from_str(r#""condition is if""#).unwrap())
				)
			),
			Some(vec![
				ConditionalBlock::new(
					Expression::Binary(Binary::new(
						Expression::Variable(Variable::from_str("condition").unwrap()),
						Token::Equal,
						Expression::Literal(Literal::from_str(r#""elseif01""#).unwrap())
					).unwrap()),
					Stmt::Print(Expression::Literal(
						Literal::from_str(r#""condition is elseif01""#).unwrap())
					)
				),
				ConditionalBlock::new(
					Expression::Binary(Binary::new(
						Expression::Variable(Variable::from_str("condition").unwrap()),
						Token::Equal,
						Expression::Literal(Literal::from_str(r#""elseif02""#).unwrap())
					).unwrap()),
					Stmt::Print(Expression::Literal(
						Literal::from_str(r#""condition is elseif02""#).unwrap())
					)
				),
			]),
			Some(Stmt::Print(Expression::Literal(
				Literal::from_str(r#""condition is else""#).unwrap())
			))
		))
	);
}

#[test]
fn empty_if_block() {
	test_parser( // {{if condition == "if"}}{{fi}}
		vec![
			Token::DelimiterStart,
			Token::If,
			Token::Value("condition"),
			Token::Equal,
			Token::Value(r#""if""#),
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
	);
}
