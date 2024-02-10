mod macros;
mod model;
mod parser;
mod lexer;
mod syntax_tree;
mod tree_walker;
mod interpreter;

// use lexer::{Lexer, Token};

pub use parser::Parser;
pub use interpreter::Interpreter;

#[cfg(test)]
mod tests;

// type Json = serde_json::Value;
// type JsonRef<'a> = &'a Json;

// TODO remove
pub use lexer::{Token, Lexer};
