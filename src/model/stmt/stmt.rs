use super::*;

/// Statements
///
/// A statement is a piece of code that can be executed.
///
/// <div class="warning">
///		Statements that begin with a special keyword (such as if, for, while, etc.), must have the
///		keyword directly after the delimiter, without any space in between.
///	</div>
///
/// # code / block
/// A block is a group of statements.
///
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// assert_eq!(osmia.run_code(r#"
/// Hello world!
/// 1 + 1 = {{ 1 + 1 }}
/// {{ "uppercase"?upper() }}
/// "#.trim()).unwrap(),
/// "Hello world!\n1 + 1 = 2\nUPPERCASE".to_string());
/// ```
///
/// # expr
/// Allows to output an [Expr](/osmia/#expression).
///
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// assert_eq!(osmia.run_code("{{ math.PI }}").unwrap(), "3.141592653589793".to_string());
/// assert_eq!(osmia.run_code("{{ 1 + 1 }}").unwrap(), "2".to_string());
/// ```
///
/// # print
/// Allows to print in the stdout the given [Expr](/osmia/#expression).
///
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// osmia.run_code("{{print 1 + 1 }}").unwrap(); // prints "2"
/// ```
///
/// # comment
/// Allows to add a comment to the code. It is ignored while interpreting the code.
///
/// <div class="warning">
/// 	If there is an starting delimiter `{{` inside a comment, an ending delimiter 
/// 	`}}` is also expected inside the comment.
/// </div>
///
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// assert_eq!(osmia.run_code(r#"{{# This is a comment }}"#).unwrap(), "".to_string());
/// assert_eq!(osmia.run_code(r#"{{#
///   This is a multi line comment.
///   Code can be added here and it will be ignored as long as there is an ending delimiter
///   {{ 1 + 1 }}
/// }}"#).unwrap(), "".to_string());
/// assert!(osmia.run_code(r#"{{#
///   {{ 1 + 1
/// }}"#).is_err());
/// ```
///
/// # assign
/// Allows to assign an [Expr](/osmia/#expression) to a variable in the
/// [Context](/osmia/#context).
///
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// assert_eq!(osmia.run_code("{{ name = \"Marvin\" }}").unwrap(), "".to_string());
/// assert_eq!(osmia.run_code("{{ name = \"Marvin\" }}{{ name }}").unwrap(), "Marvin".to_string());
/// assert_eq!(osmia.run_code("{{ name }}").unwrap(), "Marvin".to_string()); // Stored between calls
/// ```
///
/// # if, elseif, else
/// Executes the [Block](#code--block) based on boolean conditionals.
///
/// ```rust
/// use osmia::Osmia;
///
/// let code = r#"
/// {{if name == "Marvin"}}
///		Hello Marvin!
/// {{elseif name == "R2D2"}}
///		You are not Marvin, but you are R2D2!
///	{{else}}
///		You are neither Marvin nor R2D2.
///	{{fi}}
///	"#.trim();
/// let mut osmia = Osmia::try_from(r#"{ "name": "Marvin" }"#).unwrap();
/// assert_eq!(osmia.run_code(code).unwrap(), "\tHello Marvin!\n".to_string());
/// let mut osmia = Osmia::try_from(r#"{ "name": "R2D2" }"#).unwrap();
/// assert_eq!(osmia.run_code(code).unwrap(), "\tYou are not Marvin, but you are R2D2!\n".to_string());
/// let mut osmia = Osmia::try_from(r#"{ "name": "C3PO" }"#).unwrap();
/// assert_eq!(osmia.run_code(code).unwrap(), "\tYou are neither Marvin nor R2D2.\n".to_string());
/// ```
///
/// # while
/// Executes the [Block](#code--block) while the condition is true.
///
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// assert_eq!(osmia.run_code(r#"{{i = 0}}{{while i < 10}}{{i}}{{i = i + 1}}{{done}}"#).unwrap(), "0123456789".to_string());
/// ```
///
/// # for
/// Executes the [Block](#code--block) for each item in the [Expr](/osmia/#expression).
///
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// assert_eq!(osmia.run_code(r#"{{for i in [1, 2, 3]}}{{i}}{{done}}"#).unwrap(), "123".to_string());
/// ```
///
/// # break
/// Breaks out of the current [Block](#code--block).
///
/// ```rust
/// use osmia::Osmia;
///
/// let code = r#"
/// {{for i in [1, 2, 3]}}
///		{{if i == 2}}
///			{{break}}
///		{{fi}}
///		{{i}}
///	{{done}}
///	"#.trim();
///	let mut osmia = Osmia::default();
///	assert_eq!(osmia.run_code(code).unwrap(), "\t1\n".to_string());
/// ```
///
/// # continue
/// Continues with next iteration of the current [Block](#code--block).
///
/// ```rust
/// use osmia::Osmia;
///
/// let code = r#"
/// {{for i in [1, 2, 3]}}
/// 	{{if i == 2}}
/// 		{{continue}}
/// 	{{fi}}
/// 	{{i}}
/// {{done}}
/// "#.trim();
/// let mut osmia = Osmia::default();
/// assert_eq!(osmia.run_code(code).unwrap(), "\t1\n\t3\n".to_string());
/// ```
///
/// # function
/// Defines a new [Function](#function).
///
/// ```rust
/// use osmia::Osmia;
///
/// let code = r#"
/// {{fn custom_add; a, b}}
///		{{return a + b}}
///	{{done}}
///	{{custom_add(1, 2)}}
///	"#.trim();
/// let mut osmia = Osmia::default();
///	assert_eq!(osmia.run_code(code).unwrap(), "3".to_string());
/// ```
///
/// # return
/// Returns from the given [Expr](/osmia/#expression) of the current [Function](#function).
///
/// ```rust
/// use osmia::Osmia;
///
/// let code = r#"
/// {{fn custom_add; a, b}}
///		{{return a + b}}
///	{{done}}
///	{{custom_add(1, 2)}}
///	"#.trim();
/// let mut osmia = Osmia::default();
///	assert_eq!(osmia.run_code(code).unwrap(), "3".to_string());
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
	Block(Block),
	Raw(String),
	NonPrintable(String),
	NewLine,
	NewLineNonPrintable,
	Expr(Expr),
	Print(Print),
	Comment(String),
	Assign(Assign),
	If(If),
	While(While),
	For(For),
	Break,
	Continue,
	Return(Return),
	Function(Function),
}

impl Stmt {
	pub fn new_raw(s: &str) -> Self {
		Self::Raw(s.to_string())
	}

	#[cfg(debug_assertions)]
	pub fn new_non_printable(s: &str) -> Self {
		Self::NonPrintable(s.to_string())
	}

	pub fn new_print(expr: Expr) -> Self {
		Self::Print(Print::new(expr))
	}

	pub fn new_comment(s: &str) -> Self {
		Self::Comment(s.to_string())
	}

	pub fn new_assign(var: Variable, expr: Expr) -> Self {
		Self::Assign(Assign::new(var, expr))
	}

	pub fn new_return(expr: Option<Expr>) -> Self {
		Self::Return(Return::new(expr))
	}
}

impl From<Block> for Stmt {
	fn from(block: Block) -> Self {
		Self::Block(block)
	}
}

impl From<Expr> for Stmt {
	fn from(expr: Expr) -> Self {
		Self::Expr(expr)
	}
}

impl From<While> for Stmt {
	fn from(w: While) -> Self {
		Self::While(w)
	}
}
