#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
	DelimiterStart,
	DelimiterEnd,

	Raw(&'a str),

	Value(&'a str),

	// Statements
	Print,
	Assign,
	AssignEq,

	// Conditionals
	If,
	ElseIf,
	Else,

	// Loops
	While,
	For,
	In,

	// TODO break
	// TODO continue

	// Equality
	Equal,
	NotEqual,

	// Comparison
	LessThan,
	LessEqual,
	GreaterThan,
	GreaterEqual,

	// Terms
	Plus,
	Minus, // Unary

	// Factors
	Multiply,
	Divide,
	Modulo,

	Not, // Unary

	And,
	Or,

	GroupingStart,
	GroupingEnd,
}

impl Token<'_> {
	pub fn from_str(s: &str) -> Option<Token> {
		match s {
			// Delimiters
			"print" => Some(Token::Print),
			"assign" => Some(Token::Assign),
			"=" => Some(Token::AssignEq),
			"while" => Some(Token::While),
			"for" => Some(Token::For),
			"in" => Some(Token::In),
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
			"!" => Some(Token::Not),
			"&&" => Some(Token::And),
			"||" => Some(Token::Or),
			"(" => Some(Token::GroupingStart),
			")" => Some(Token::GroupingEnd),
			_ => None
		}
	}

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
			Token::DelimiterStart => write!(f, "{{{{"),
			Token::DelimiterEnd => write!(f, "}}}}"),
			Token::Raw(s) => write!(f, "{}", s),
			Token::Value(s) => write!(f, "{}", s),
			Token::Print => write!(f, "print"),
			Token::Assign => write!(f, "assign"),
			Token::AssignEq => write!(f, "="),
			Token::If => write!(f, "if"),
			Token::ElseIf => write!(f, "elseif"),
			Token::Else => write!(f, "else"),
			Token::While => write!(f, "while"),
			Token::For => write!(f, "for"),
			Token::In => write!(f, "in"),
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
			Token::Not => write!(f, "!"),
			Token::And => write!(f, "&&"),
			Token::Or => write!(f, "||"),
			Token::GroupingStart => write!(f, "("),
			Token::GroupingEnd => write!(f, ")"),
		}
	}
}
