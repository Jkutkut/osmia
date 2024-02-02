mod parser;
mod lexer;
mod syntax_tree;

// use lexer::{Lexer, Token};

pub use parser::Parser;

#[cfg(test)]
mod tests;

// type Json = serde_json::Value;
// type JsonRef<'a> = &'a Json;

// TODO remove
pub use lexer::{Token, Lexer};
