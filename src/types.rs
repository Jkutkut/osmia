use crate::model::lexer::Token;
use crate::model::code::Stmt;

/// The type of the lexer's output.
pub type LexerCode = Vec<Token>;

/// Parsing documentation: // TODO
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
/// unary          → ( "!" | "-" | "+" )* method_call
/// method_call    → primary ( "?" call )*
/// primary        → literal | call | array | object | grouping
/// literal        → float | int | string | boolean | null
/// call           → variable ( "(" arguments? ")" )*
/// arguments      → expression ( "," expression )*
/// variable       → obj
/// obj            → arr ( "." arr )*
/// arr            → identifier ( "[" expression "]" )*
/// array          → "[" ( expression? ( "," expression )* )? "]"
/// object         → "{" ( object_entry ( "," object_entry )* )? "}"
/// object_entry   → expression ":" expression
/// grouping       → "(" expression ")"
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

/// TODO
pub type OsmiaOutput = String;

/// TODO
pub type OsmiaError = String;

pub use crate::ctx::Ctx;
