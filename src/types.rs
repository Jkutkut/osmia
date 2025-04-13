use crate::model::lexer::Token;
use crate::model::stmt::Stmt;
use std::cell::RefCell;

/// The type of the lexer's output.
pub type LexerCode = Vec<Token>;

/// Parsing documentation:
///
/// ## Structure:
/// ```text
/// program        → stmt
/// stmt           → block | raw | evaluation | print | comment | assign |
///                  if | while | foreach |
///                  break | continue | return |
///                  function
///
/// block          → ( stmt )*
/// raw            → "..."
/// evaluation     → "{{" expression "}}"
/// print          → "{{" "print" expression "}}"
/// comment        → "{{" "#" expression "}}"
/// assign         → "{{" variable "=" expression "}}"
/// if             → "{{" "if" conditional ( elseif )* ( else )? "{{" "fi" "}}"
/// elseif         → "{{" "elseif" conditional
/// else           → "{{" "else" block
/// conditional    → expression "}}" stmt
/// while          → "{{" "while" conditional "done" "}}"
/// for            → "{{" "for" identifier "in" iterable "}}" stmt "{{" "done" "}}"
/// iterable       → expression
/// break          → "{{" "break" "}}"
/// continue       → "{{" "continue" "}}"
/// return         → "{{" "return" expression? "}}"
/// function       → "{{" "fn" identifier ( ";" parameters )? "}}" block "{{" "done" "}}"
/// parameters     → parameter ( "," parameter )* ( "," "..." identifier)?
/// parameter      → identifier ( "=" expression )?
///
/// expression     → lambda | logic_or
/// lambda         → "fn" "(" parameters? ")" "=>" expression
/// logic_or       → logic_and ( "||" logic_and )*
/// logic_and      → equality ( "&&" equality )*
/// equality       → bitwise ( ( "!=" | "==" ) bitwise )*
/// bitwise        → comparison ( ( "&" | "|" | "^" ) comparison )*
/// comparison     → bitshift ( ( ">" | ">=" | "<" | "<=" ) bitshift )*
/// bitshift       → term ( ( ">>" | "<<" ) term )*
/// term           → factor ( ( "-" | "+" ) factor )*
/// factor         → unary ( ( "/" | "*" | "%" ) unary )*
/// unary          → ( "!" | "-" | "+" )* value
///
/// value          → primary ( method_call | call | variable )*
/// primary        → array | object | grouping | literal | var_name
/// array          → "[" ( expression? ( "," expression )* )? "]"
/// object         → "{" ( object_entry ( "," object_entry )* )? "}"
/// object_entry   → expression ":" expression
/// grouping       → "(" expression ")"
/// literal        → float | int | string | boolean | null
/// var_name       → identifier
///
/// method_call    → ( "?" identifier call )*
/// call           → ( "(" arguments? ")" )*
/// arguments      → expression ( "," expression )*
/// variable       → ( ( "[" expression "]" ) | ( "." identifier ) )*
///
/// identifier     → alpha ( alpha | digit )*
/// alpha          → [a-zA-Z_]
/// digit          → [0-9]
/// int            → digit+
/// float          → int "." int
/// boolean        → "true" | "false"
/// null           → "null"
/// string         → '"' ( [^"] )* '"' | "'" ( [^'] )* "'"
/// ```
pub type ParserCode = Stmt;

/// The type of the interpreter's output.
pub type OsmiaOutput = String;

/// The type of the interpreter's error.
pub type OsmiaError = String;

pub use crate::ctx::Ctx;
pub type CtxRef<'a> = RefCell<&'a mut Ctx>;
