#[derive(Debug)]
pub enum Token<'a> {
	DelimiterStart,
	DelimiterEnd,

	Raw(&'a str),
	Value(&'a str),

	If,
	ElseIf,
	Else,

	For,

	Equal,
	NotEqual,
	LessThan,
	LessEqual,
	GreaterThan,
	GreaterEqual,

	Plus,
	Minus,
	Multiply,
	Divide,
	Modulo,

	And,
	Or,
	Not,

	GroupingStart,
	GroupingEnd,

	// Assign,
}

impl Token<'_> {
	pub fn from_str(s: &str) -> Option<Token> {
		match s {
			"if" => Some(Token::If),
			"elif" => Some(Token::ElseIf),
			"else" => Some(Token::Else),
			"for" => Some(Token::For),
			"==" => Some(Token::Equal),
			"!=" => Some(Token::NotEqual),
			"<" => Some(Token::LessThan),
			"<=" => Some(Token::LessEqual),
			">" => Some(Token::GreaterThan),
			">=" => Some(Token::GreaterEqual),
			"+" => Some(Token::Plus),
			"-" => Some(Token::Minus),
			"*" => Some(Token::Multiply),
			"/" => Some(Token::Divide),
			"%" => Some(Token::Modulo),
			"&&" => Some(Token::And),
			"||" => Some(Token::Or),
			"!" => Some(Token::Not),
			"(" => Some(Token::GroupingStart),
			")" => Some(Token::GroupingEnd),
			_ => None
		}
	}

// 	fn is_conditional(&self) -> bool {
// 		match self {
// 			Token::If => true,
// 			Token::ElseIf => true,
// 			Token::Else => true,
// 			Token::For => true,
// 			_ => false
// 		}
// 	}
}
