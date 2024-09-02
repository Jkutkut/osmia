use crate::model::lexer::Token;
use crate::macro_tests;

#[cfg(test)]
fn test_debug_token(
	token: Token,
	expected: &str
) {
	let actual = format!("{:?}", token);
	assert_eq!(actual, expected);
}

#[cfg(test)]
fn combined_test(
	token: Token,
	s: &str
) {
	match Token::try_from(s) {
		Ok(actual) => assert_eq!(actual, token.clone()),
		Err(e) => panic!("{}", e)
	}
	test_debug_token(token, s);
}

macro_tests! {
	test_debug_token,
	// Complex
	(debug_token_raw, Token::Raw("This is raw".to_string()), "Raw(This is raw)"),
	(debug_token_str, Token::Str("string".to_string()), r#"Str("string")"#),
	(debug_token_alpha, Token::Alpha("abcd".to_string()), "Alpha(abcd)"),
	(debug_token_int, Token::Number("123".to_string()), "Number(123)"),
	(debug_token_float, Token::Number("123.123".to_string()), "Number(123.123)"),
	(debug_token_bool_true, Token::Bool(true), "Bool(true)"),
	(debug_token_bool_false, Token::Bool(false), "Bool(false)"),
}

macro_tests! {
	combined_test,

	// Delimiters
	(debug_token_stmtstart, Token::StmtStart, "{{"),
	(debug_token_stmtend, Token::StmtEnd, "}}"),
	(debug_token_objectstart, Token::ObjectStart, "{"),
	(debug_token_objectend, Token::ObjectEnd, "}"),
	(debug_token_parentstart, Token::ParentStart,"("),
	(debug_token_parentend, Token::ParentEnd,")"),
	(debug_token_arraystart, Token::ArrayStart,"["),
	(debug_token_arrayend, Token::ArrayEnd,"]"),

	// Statements
	(debug_token_print, Token::Print,"print"),
	(debug_token_assign, Token::Assign,"assign"),
	(debug_token_comment, Token::Comment, "#"),
	(debug_token_function, Token::Function,"fn"),
	(debug_token_return, Token::Return,"return"),

	// Conditionals
	(debug_token_if, Token::If,"if"),
	(debug_token_elseif, Token::ElseIf,"elif"),
	(debug_token_else, Token::Else,"else"),
	(debug_token_fi, Token::Fi,"fi"),

	// Loops
	(debug_token_while, Token::While,"while"),
	(debug_token_for, Token::For,"for"),
	(debug_token_in, Token::In,"in"),
	(debug_token_continue, Token::Continue,"continue"),
	(debug_token_break, Token::Break,"break"),
	(debug_token_done, Token::Done,"done"),

	// Equality
	(debug_token_assigneq, Token::AssignEq,"="),
	(debug_token_equal, Token::Equal,"=="),
	(debug_token_notequal, Token::NotEqual,"!="),

	// Bitwise
	(debug_token_bitand, Token::BitAnd,"&"),
	(debug_token_bitor, Token::BitOr,"|"),
	(debug_token_bitxor, Token::BitXor,"^"),

	// Comparison
	(debug_token_greater, Token::Greater,">"),
	(debug_token_greaterequal, Token::GreaterEqual,">="),
	(debug_token_less, Token::Less,"<"),
	(debug_token_lessequal, Token::LessEqual,"<="),

	// Bitshift
	(debug_token_bitshiftleft, Token::BitShiftLeft,"<<"),
	(debug_token_bitshiftright, Token::BitShiftRight,">>"),

	// Arithmetic
	(debug_token_plus, Token::Plus,"+"),
	(debug_token_minus, Token::Minus,"-"),
	(debug_token_mult, Token::Mult,"*"),
	(debug_token_div, Token::Div,"/"),
	(debug_token_mod, Token::Mod,"%"),

	// Logical
	(debug_token_not, Token::Not,"!"),
	(debug_token_and, Token::And,"&&"),
	(debug_token_or, Token::Or,"||"),

	// Primary
	(debug_token_null, Token::Null,"null"),

	// Special
	(debug_token_dot, Token::Dot,"."),
	(debug_token_comma, Token::Comma,","),
	(debug_token_colon, Token::Colon,":"),
	(debug_token_semicolon, Token::Semicolon,";"),
	(debug_token_spread, Token::Spread,"..."),
	(debug_token_question, Token::Question,"?"),
	(debug_token_arrow, Token::Arrow, "=>"),
}
