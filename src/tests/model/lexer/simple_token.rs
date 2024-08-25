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
	(keyword_tokens, "print assign fn return if elseif else fi while for in continue break done true false null", vec![
		Token::Print, Token::Assign,
		Token::Function, Token::Return,
		Token::If, Token::ElseIf, Token::Else, Token::Fi,
		Token::While, Token::For, Token::In,
		Token::Continue, Token::Break, Token::Done,
		Token::Bool(true), Token::Bool(false), Token::Null
	]),
);

macro_tests!(
	lexer_expression_test_fail,
);
