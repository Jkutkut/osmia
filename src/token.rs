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

	// pub fn is_conditional(&self) -> bool {
	// 	match self {
	// 		Token::If => true,
	// 		Token::ElseIf => true,
	// 		Token::Else => true,
	// 		Token::For => true,
	// 		_ => false
	// 	}
	// }

	pub fn is_binary_operator(&self) -> bool {
		match self {
			Token::Equal => true,
			Token::NotEqual => true,
			Token::LessThan => true,
			Token::LessEqual => true,
			Token::GreaterThan => true,
			Token::GreaterEqual => true,
			Token::Plus => true,
			Token::Minus => true,
			Token::Multiply => true,
			Token::Divide => true,
			Token::Modulo => true,
			Token::And => true,
			Token::Or => true,
			_ => false
		}
	}

	pub fn is_uniary(&self) -> bool {
		match self {
			Token::Minus => true,
			Token::Not => true,
			_ => false
		}
	}
}

impl std::fmt::Display for Token<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::DelimiterStart => write!(f, "{{"),
			Token::DelimiterEnd => write!(f, "}}"),
			Token::Raw(s) => write!(f, "{}", s),
			Token::Value(s) => write!(f, "{}", s),
			Token::If => write!(f, "if"),
			Token::ElseIf => write!(f, "elif"),
			Token::Else => write!(f, "else"),
			Token::For => write!(f, "for"),
			Token::Equal => write!(f, "=="),
			Token::NotEqual => write!(f, "!="),
			Token::LessThan => write!(f, "<"),
			Token::LessEqual => write!(f, "<="),
			Token::GreaterThan => write!(f, ">"),
			Token::GreaterEqual => write!(f, ">="),
			Token::Plus => write!(f, "+"),
			Token::Minus => write!(f, "-"),
			Token::Multiply => write!(f, "*"),
			Token::Divide => write!(f, "/"),
			Token::Modulo => write!(f, "%"),
			Token::And => write!(f, "&&"),
			Token::Or => write!(f, "||"),
			Token::Not => write!(f, "!"),
			Token::GroupingStart => write!(f, "("),
			Token::GroupingEnd => write!(f, ")"),
		}
	}
}
