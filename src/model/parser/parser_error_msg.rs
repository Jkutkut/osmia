use crate::OsmiaError;
use crate::model::lexer::Token;
use super::osmia_parser::OsmiaParserImpl;

pub enum ParserErrorMsg {
	MissingKeyword(Token),
	Custom(String),
	Unclosed(String, Token),
	Expected(Token),
	ParseValue(String),
}

impl ParserErrorMsg {
	pub fn report(&self, paser: &OsmiaParserImpl) -> OsmiaError {
		match self {
			ParserErrorMsg::MissingKeyword(token) => format!(
				"Expected keyword '{:?}' but got '{:?}'",
				token, paser.get_current()
			).into(),
			ParserErrorMsg::Custom(msg) => format!(
				"{} {:?}",
				msg, paser.get_current()
			).into(),
			ParserErrorMsg::Unclosed(block_type, expected) => format!(
				"Unclosed {}. Expected '{:?}' but got '{:?}'",
				block_type, expected, paser.get_current()
			),
			ParserErrorMsg::Expected(token) => format!(
				"Expected '{:?}' but got '{:?}'",
				token, paser.get_current()
			),
			ParserErrorMsg::ParseValue(parse_type) => format!(
				"Not able to parse {:?} as a {}",
				paser.get_current(), parse_type
			),
		}
	}
}
