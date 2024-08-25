#[derive(Clone, PartialEq)]
pub enum Token {
	Raw(String),

	// File
	NewLine,
	Eof,

	// Delimiters
	StmtStart,
	StmtEnd,
	ParentStart,
	ParentEnd,
	ObjectStart,
	ObjectEnd,
	ArrayStart,
	ArrayEnd,

	// Statements
	Print,
	Assign,
	Comment,
	Function,
	Return,

	// Conditionals
	If,
	ElseIf,
	Else,
	Fi,

	// Loops
	While,
	For,
	In,
	Continue,
	Break,
	Done,

	// Equality
	AssignEq,
	Equal,
	NotEqual,

	// Bitwise
	BitAnd,
	BitOr,
	BitXor,

	// Comparison
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// Bitshift
	BitShiftLeft,
	BitShiftRight,

	// Arithmetic
	Plus,
	Minus,
	Mult,
	Div,
	Mod,

	// Logical
	Not,
	And,
	Or,

	// Primary
	Str(String),
	Alpha(String),
	Number(String),
	Bool(bool),
	Null,

	// Special
	Dot,
	Comma,
	Colon,
	SemiColon,
	Spread,
	Question,
}

impl Token {
	pub fn new_raw(s: &str) -> Self {
		Self::Raw(s.to_string())
	}

	pub fn new_str(s: &str) -> Self {
		Self::Str(s.to_string())
	}

	pub fn new_alpha(s: &str) -> Self {
		Self::Alpha(s.to_string())
	}

	pub fn new_number(s: &str) -> Self {
		Self::Number(s.to_string())
	}
}

macro_rules! impl_token_traits {
	(
		$( $variant:ident <=> $str_value:expr $(,)? ),*
	) => {
		impl TryFrom<&str> for Token {
			type Error = String;
			fn try_from(s: &str) -> Result<Self, Self::Error> {
				Ok(match s {
					"{{" => Self::StmtStart,
					"}}" => Self::StmtEnd,
					"{" => Self::ObjectStart,
					"}" => Self::ObjectEnd,
					$($str_value => Self::$variant,)*
					_ => return Err(format!("Could not convert {:?} to a token automatically", s)),
				})
			}
		}

		crate::impl_debug!(
			Token,
			(
				// Format values
				StmtStart <=> "{{{{",
				StmtEnd <=> "}}}}",
				ObjectStart <=> "{{",
				ObjectEnd <=> "}}",

				// Complex values
				Raw(s) <=> "Raw({s})",
				Str(s) <=> "Str({s:?})",
				Alpha(s) <=> "Alpha({s})",
				Number(s) <=> "Number({s})",
				Bool(b) <=> "Bool({b})",
				$( $variant <=> $str_value ),*
			)
		);
	};
}

impl_token_traits!(
	// File
	NewLine <=> "\\n",
	Eof <=> "Eof",

	// Delimiters
	ParentStart <=> "(",
	ParentEnd <=> ")",
	ArrayStart <=> "[",
	ArrayEnd <=> "]",

	// Statements
	Print <=> "print",
	Assign <=> "assign",
	Comment <=> "#",
	Function <=> "fn",
	Return <=> "return",

	// Conditionals
	If <=> "if",
	ElseIf <=> "elif",
	Else <=> "else",
	Fi <=> "fi",

	// Loops
	While <=> "while",
	For <=> "for",
	In <=> "in",
	Continue <=> "continue",
	Break <=> "break",
	Done <=> "done",

	// Equality
	AssignEq <=> "=",
	Equal <=> "==",
	NotEqual <=> "!=",

	// Bitwise
	BitAnd <=> "&",
	BitOr <=> "|",
	BitXor <=> "^",

	// Comparison
	Greater <=> ">",
	GreaterEqual <=> ">=",
	Less <=> "<",
	LessEqual <=> "<=",

	// Bitshift
	BitShiftLeft <=> "<<",
	BitShiftRight <=> ">>",

	// Arithmetic
	Plus <=> "+",
	Minus <=> "-",
	Mult <=> "*",
	Div <=> "/",
	Mod <=> "%",

	// Logical
	Not <=> "!",
	And <=> "&&",
	Or <=> "||",

	// Primary
	Null <=> "null",

	// Special
	Dot <=> ".",
	Comma <=> ",",
	Colon <=> ":",
	SemiColon <=> ";",
	Spread <=> "...",
	Question <=> "?",
);
