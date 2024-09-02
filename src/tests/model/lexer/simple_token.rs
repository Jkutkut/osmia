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
			Token::AssignEq, Token::Whitespace, Token::Equal, Token::Whitespace, Token::NotEqual, Token::Whitespace,
			// Bitwise
			Token::BitAnd, Token::Whitespace, Token::BitOr, Token::Whitespace, Token::BitXor, Token::Whitespace,
			// Comparison
			Token::Greater, Token::Whitespace, Token::GreaterEqual, Token::Whitespace,
			Token::Less, Token::Whitespace, Token::LessEqual, Token::Whitespace,
			// Bitshift
			Token::BitShiftLeft, Token::Whitespace, Token::BitShiftRight, Token::Whitespace,
			// Arithmetic
			Token::Plus, Token::Whitespace, Token::Minus, Token::Whitespace,
			Token::Mult, Token::Whitespace, Token::Div, Token::Whitespace,
			Token::Mod, Token::Whitespace,
			// Logical
			Token::Not, Token::Whitespace, Token::And, Token::Whitespace, Token::Or,
		]
	),
	(simple_tokens_special, ". , : ; ... ? =>", vec![
		Token::Dot, Token::Whitespace, Token::Comma, Token::Whitespace,
		Token::Colon, Token::Whitespace, Token::Semicolon, Token::Whitespace,
		Token::Spread, Token::Whitespace, Token::Question, Token::Whitespace, Token::Arrow,
	]),
	(keyword_tokens, "print assign fn return if elseif else fi while for in continue break done true false null", vec![
		Token::Print, Token::Whitespace, Token::Assign, Token::Whitespace,
		Token::Function, Token::Whitespace, Token::Return, Token::Whitespace,
		Token::If, Token::Whitespace, Token::ElseIf, Token::Whitespace, Token::Else, Token::Whitespace, Token::Fi, Token::Whitespace,
		Token::While, Token::Whitespace, Token::For, Token::Whitespace, Token::In, Token::Whitespace,
		Token::Continue, Token::Whitespace, Token::Break, Token::Whitespace, Token::Done, Token::Whitespace,
		Token::Bool(true), Token::Whitespace, Token::Bool(false), Token::Whitespace, Token::Null
	]),
);

macro_tests!(
	lexer_expression_test_fail,
);
