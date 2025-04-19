use crate::model::lexer::Token;
use crate::model::stmt::Stmt;
use std::cell::RefCell;

/// The type of the lexer's output.
pub type LexerCode = Vec<Token>;

/// ## Structure:
#[doc = include_str!("../docs/parsing_structure.md")]
pub type ParserCode = Stmt;

/// The type of the interpreter's output.
pub type OsmiaOutput = String;

/// The type of the interpreter's error.
pub type OsmiaError = String;

pub use crate::ctx::Ctx;
pub type CtxRef<'a> = RefCell<&'a mut Ctx>;
