use super::{
	Lexer,
	OsmiaLexerScanner
};
use crate::LexerCode;
use crate::OsmiaError;
use crate::model::lexer::Token;

pub struct OsmiaLexer {}

impl OsmiaLexer {
	pub fn new() -> Self {
		Self {}
	}
}

impl Lexer<LexerCode, OsmiaError> for OsmiaLexer {
	fn lex(&self, code: &str) -> Result<LexerCode, OsmiaError> {
		let mut scanner = OsmiaLexerScanner::new(code);
		match scanner.scan() {
			Err(err) => Err(format!("Lexer error: {}", err)),
			Ok(tokens) => Ok(Self::clean_tokens(tokens)),
		}
	}
}

impl OsmiaLexer {
	fn clean_tokens(mut tokens: LexerCode) -> LexerCode {
		let mut i: usize = tokens.len() - 1;
		let mut line_end: usize;
		loop {
			line_end = i;
			i = Self::find_start_line(&tokens, line_end);
			if !Self::is_printable_line(&tokens, i, line_end) {
				if matches!(tokens[line_end], Token::NewLine) {
					tokens[line_end] = Token::NewLineNonPrintable;
				}
				let mut j = i;
				while j < line_end && Self::is_whitespace_token(&tokens[j]) {
					tokens[j] = Token::NonPrintable(tokens[j].as_raw_str().unwrap().into());
					j += 1;
				}
			}
			if i == 0 {
				break;
			}
			i -= 1;
		}
		tokens
	}

	fn find_start_line(tokens: &LexerCode, end_line: usize) -> usize {
		if end_line == 0 || tokens.is_empty() {
			return 0;
		}
		let mut i = end_line - 1;
		while i > 0 && !matches!(tokens[i], Token::NewLine) {
			i -= 1;
		}
		if matches!(tokens[i], Token::NewLine) && i < end_line {
			i += 1;
		}
		return i;
	}

	fn is_printable_line(tokens: &LexerCode, mut start: usize, mut end: usize) -> bool {
		if start >= end {
			return true;
		}
		// indenting
		while start < end && Self::is_whitespace_token(&tokens[start]) {
			start += 1;
		}
		// non-printable stmts
		while start < end {
			(start, end) = match Self::walk_non_printable_stmt(tokens, start, end) {
				None => return true,
				Some(p) => p
			};
			assert!(start == end || matches!(tokens[start], Token::StmtEnd));
			start += 1;
		}
		false
	}

	fn walk_non_printable_stmt(tokens: &LexerCode, mut start: usize, end: usize) -> Option<(usize, usize)> {
		if !matches!(tokens[start], Token::StmtStart) {
			return None;
		}
		start += 1;
		while start < end && matches!(tokens[start], Token::Whitespace) {
			start += 1;
		}
		match tokens[start] {
			Token::Print | Token::Comment | Token::Function | Token::Return => (),
			Token::If | Token::ElseIf | Token::Else | Token::Fi => (),
			Token::While | Token::For | Token::Continue | Token::Break | Token::Done => (),
			_ => return None
		}
		while start < end && !matches!(tokens[start], Token::StmtEnd) {
			start += 1;
		}
		Some((start, end))
	}

	fn is_whitespace_token(token: &Token) -> bool {
		match token {
			Token::Raw(r) => r.chars().all(|c| c.is_ascii_whitespace()),
			_ => false
		}
	}
}
