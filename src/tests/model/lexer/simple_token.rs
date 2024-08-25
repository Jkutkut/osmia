use super::*;

macro_tests!(
	lexer_expression_test,
	(simple_tokens_delimiters, "(){}[]", vec![
		Token::ParentStart, Token::ParentEnd,
		Token::ObjectStart, Token::ObjectEnd,
		Token::ArrayStart, Token::ArrayEnd,
	]),
	(simple_tokens_operators,
		"= == != & | ^ > >= < <= << >> + - * / % ! && ||",
		vec![
			// Equality
			Token::AssignEq, Token::Equal, Token::NotEqual,
			// Bitwise
			Token::BitAnd, Token::BitOr, Token::BitXor,
			// Comparison
			Token::Greater, Token::GreaterEqual,
			Token::Less, Token::LessEqual,
			// Bitshift
			Token::BitShiftLeft, Token::BitShiftRight,
			// Arithmetic
			Token::Plus, Token::Minus,
			Token::Mult, Token::Div,
			Token::Mod,
			// Logical
			Token::Not, Token::And, Token::Or,
		]
	),
	(simple_tokens_special, "# . , : ; ... ?", vec![
		Token::Comment,
		Token::Dot, Token::Comma,
		Token::Colon, Token::SemiColon,
		Token::Spread, Token::Question,
	]),
);

macro_tests!(
	lexer_expression_test_fail,
);
