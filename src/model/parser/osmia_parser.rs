use super::Parser;
use crate::types::{
	LexerCode,
	ParserCode,
	OsmiaError,
};
use crate::model::code::*;
use crate::model::ctx::JsonTreeKey;

pub struct OsmiaParser;

impl OsmiaParser {
	pub fn new() -> Self {
		Self
	}
}

impl Parser<LexerCode, ParserCode, OsmiaError> for OsmiaParser {
	fn parse(&self, code: LexerCode) -> Result<ParserCode, OsmiaError> {
		Ok(OsmiaParserImpl::new(code).parse()?)
	}
}

use crate::model::lexer::Token;

struct OsmiaParserImpl {
	line: usize,
	code: LexerCode,
	current: usize,
}

impl OsmiaParserImpl {
	pub fn new(code: LexerCode) -> Self {
		Self {
			line: 1, code,
			current: 0
		}
	}

	pub fn parse(&mut self) -> Result<ParserCode, OsmiaError> {
		let code = self.code()?;
		if !self.done() {
			return Err(self.error(&format!(
				"Unexpected token {:?}",
				self.get_current()
			)));
		}
		Ok(code)
	}

	fn code(&mut self) -> Result<ParserCode, OsmiaError> {
		self.block()
	}
}

impl OsmiaParserImpl {
	fn error(&self, msg: &str) -> String {
		format!(
			"Parser error: Line {}: {}",
			self.line, msg
		)
	}

	fn done(&self) -> bool {
		self.check_current(&Token::Eof)
	}

	fn check_current(&self, token: &Token) -> bool {
		self.get_current() == token
	}

	fn get_current(&self) -> &Token {
		&self.code[self.current]
	}

	fn get_previous(&self) -> &Token {
		&self.code[self.current - 1]
	}

	fn advance(&mut self) -> &Token {
		if !self.done() {
			self.current += 1;
		}
		self.get_previous()
	}

	fn consume(&mut self, token: Token, error: fn(parser: &Self) -> OsmiaError) -> Result<(), OsmiaError> {
		match self.check_current(&token) {
			true => {
				self.advance();
				Ok(())
			}
			false => Err(error(self))
		}
	}

	fn match_and_advance(&mut self, types: &[Token]) -> bool {
		if self.done() {
			return false;
		}
		for token_type in types {
			if self.check_current(token_type) {
				self.advance();
				return true
			}
		}
		false
	}

	fn binary(
		&mut self,
		ops: &[Token],
		side: fn(&mut Self) -> Result<Expr, OsmiaError>,
	) -> Result<Expr, OsmiaError> {
		let mut expr = side(self)?;
		self.consume_whitespaces();
		while self.match_and_advance(ops) {
			let op: Option<BinaryOp> = self.get_previous().into();
			self.consume_whitespaces();
			expr = Binary::new(
				expr,
				op.unwrap(),
				side(self)?,
			).into();
			self.consume_whitespaces();
		}
		Ok(expr)
	}
}

impl OsmiaParserImpl {
	fn consume_new_lines(&mut self) {
		while !self.done() {
			match self.get_current() {
				Token::NewLine => self.line += 1,
				_ => return
			}
			self.advance();
		}
	}

	fn consume_whitespaces(&mut self) {
		while !self.done() {
			self.consume_new_lines();
			match self.get_current() {
				Token::Whitespace => (),
				_ => return
			};
			self.advance();
		}
	}

	fn block(&mut self) -> Result<Stmt, OsmiaError> {
		self.breakable_block(&None)
	}

	fn breakable_block(&mut self, break_with: &Option<Vec<Token>>) -> Result<Stmt, OsmiaError> {
		let mut statements: Block = Block::new();
		loop {
			self.consume_new_lines();
			if self.done() {
				break;
			}
			match self.advance() {
				Token::Raw(r) => statements.push(Stmt::Raw(r.to_string())),
				Token::StmtStart => match self.stmt(break_with)? {
					None => break,
					Some(stmt) => statements.push(stmt),
				},
				_ => return Err(self.error(&format!(
					"Unexpected token {:?}",
					self.get_current()
				)))
			}
		}
		if statements.len() == 1 {
			let mut arr: Vec<Stmt> = statements.into();
			return Ok(arr.pop().unwrap().into());
		}
		Ok(statements.into())
	}

	fn stmt(&mut self, return_none_with: &Option<Vec<Token>>) -> Result<Option<Stmt>, OsmiaError> {
		let stmt: Stmt = match self.get_current() {
			Token::Comment => self.comment()?,
			_ => self.expr_stmt()?,
		};
		self.consume(
			Token::StmtEnd,
			|parser| parser.error(&format!(
				"Expected '{:?}', got '{:?}'",
				Token::StmtEnd, parser.get_current()
			))
		)?;
		Ok(Some(stmt))
	}

	fn comment(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Comment, |parser| parser.error(&format!(
			"Expected comment, got '{:?}'",
			parser.get_current()
		)))?;
		let mut comment = String::new();
		while !self.done() && !self.check_current(&Token::StmtEnd) {
			match self.advance() {
				Token::Raw(r) => comment.push_str(r),
				Token::NewLine => {
					comment.push('\n');
					self.line += 1;
				},
				_ => return Err(self.error(&format!(
					"Expected comment, got '{:?}'",
					self.get_current()
				)))
			};
		}
		Ok(Stmt::Comment(comment))
	}

	fn expr_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume_whitespaces();
		let stmt = self.expr()?.into();
		self.consume_whitespaces();
		Ok(stmt)
	}

	fn parameters(&mut self) -> Result<(), OsmiaError> { // TODO
		todo!() // TODO
		// self.parameter()
		// ,
		// self.parameter()
		// *
		// ...
		// self.identifier()
		// ?
	}

	fn parameter(&mut self) -> Result<(), OsmiaError> { // TODO
		todo!() // TODO
		// self.identifier()
		// :
		// self.expr()
	}

	fn expr(&mut self) -> Result<Expr, OsmiaError> {
		match self.get_current() {
			Token::Function => self.lambda(),
			_ => self.logic_or()
		}
	}

	fn lambda(&mut self) -> Result<Expr, OsmiaError> {
		todo!() // TODO
	}

	fn logic_or(&mut self) -> Result<Expr, OsmiaError> {
		self.binary(&[Token::Or], |parser| parser.logic_and())
	}

	fn logic_and(&mut self) -> Result<Expr, OsmiaError> {
		self.binary(&[Token::And], |parser| parser.equality())
	}

	fn equality(&mut self) -> Result<Expr, OsmiaError> {
		self.binary(&[Token::Equal, Token::NotEqual], |parser| parser.bitwise())
	}

	fn bitwise(&mut self) -> Result<Expr, OsmiaError> {
		self.binary(&[Token::BitAnd, Token::BitOr, Token::BitXor], |parser| parser.comparison())
	}

	fn comparison(&mut self) -> Result<Expr, OsmiaError> {
		self.binary(
			&[Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual],
			|parser| parser.bitshift(),
		)
	}

	fn bitshift(&mut self) -> Result<Expr, OsmiaError> {
		self.binary(&[Token::BitShiftLeft, Token::BitShiftRight], |parser| parser.term())
	}

	fn term(&mut self) -> Result<Expr, OsmiaError> {
		self.binary(&[Token::Plus, Token::Minus], |parser| parser.factor())
	}

	fn factor(&mut self) -> Result<Expr, OsmiaError> {
		self.binary(&[Token::Mult, Token::Div, Token::Mod], |parser| parser.unary())
	}

	fn unary(&mut self) -> Result<Expr, OsmiaError> {
		if self.match_and_advance(&[Token::Not, Token::Minus, Token::Plus]) {
			let operator: Option<UnaryOp> = self.get_previous().into();
			let right = self.unary()?;
			return Ok(Unary::new(operator.unwrap(), right).into());
		}
		self.method_call()
	}

	fn method_call(&mut self) -> Result<Expr, OsmiaError> {
		self.primary() // TODO
		// if ?
		// self.call() // TODO
	}

	fn primary(&mut self) -> Result<Expr, OsmiaError> {
		match self.get_current() {
			Token::ArrayStart => self.array(),
			Token::ObjectStart => self.object(),
			Token::ParentStart => self.grouping(),
			Token::Str(_) | Token::Number(_) | Token::Bool(_) | Token::Null => self.literal(),
			_ => self.call(),
		}
	}

	fn literal(&mut self) -> Result<Expr, OsmiaError> {
		let expr = match self.get_current() {
			Token::Null => Expr::Null,
			Token::Bool(b) => Expr::Bool(*b),
			Token::Str(s) => Expr::Str(s.to_string()),
			Token::Number(n) => {
				match n.contains('.') {
					true => match n.parse::<f64>() {
						Ok(f) => Expr::Float(f),
						Err(_) => return Err(self.error(&format!(
							"Could not parse float: {}",
							n
						)))
					},
					false => match n.parse::<i64>() {
						Ok(i) => Expr::Int(i),
						Err(_) => return Err(self.error(&format!(
							"Could not parse int: {}",
							n
						)))
					}
				}
			},
			_ => unreachable!(),
		};
		self.advance();
		Ok(expr)
	}

	fn call(&mut self) -> Result<Expr, OsmiaError> {
		self.variable() // TODO
		// if (
		// self.arguments() // TODO
		// )
	}

	fn arguments(&mut self) -> Result<Expr, OsmiaError> {
		// self.expr()
		// ,
		// self.expr()
		// *
		todo!() // TODO
	}

	fn variable(&mut self) -> Result<Expr, OsmiaError> {
		Ok(self.obj()?.into()) // TODO
	}

	fn obj(&mut self) -> Result<Variable, OsmiaError> {
		let mut var = self.arr()?;
		while self.match_and_advance(&[Token::Dot]) {
			var.push(self.identifier()?.into())
		}
		Ok(var)
	}

	fn arr(&mut self) -> Result<Variable, OsmiaError> {
		let mut arr = vec![self.identifier()?.into()];
		while self.match_and_advance(&[Token::ArrayStart]) {
			arr.push(self.expr()?.into());
			self.consume(Token::ArrayEnd, |parser| parser.error(&format!(
				"Expected end of array, got: {:?}",
				parser.get_current()
			)))?;
		}
		Ok(Variable::from_vec(arr))
	}

	fn array(&mut self) -> Result<Expr, OsmiaError> {
		self.consume(Token::ArrayStart, |parser| parser.error(&format!(
			"Expected start of array, got: {:?}",
			parser.get_current()
		)))?;
		let mut arr: Array = Vec::new().into();
		self.consume_whitespaces();
		if !self.match_and_advance(&[Token::ArrayEnd]) {
			arr.push(self.expr()?.into());
			self.consume_whitespaces();
			while !self.match_and_advance(&[Token::ArrayEnd]) {
				self.consume(Token::Comma, |parser| parser.error(&format!(
					"Expected comma, got: {:?}",
					parser.get_current()
				)))?;
				self.consume_whitespaces();
				arr.push(self.expr()?.into());
				self.consume_whitespaces();
			}
		}
		Ok(arr.into())
	}

	fn object(&mut self) -> Result<Expr, OsmiaError> {
		self.consume(Token::ObjectStart, |parser| parser.error(&format!(
			"Expected start of object, got: {:?}",
			parser.get_current()
		)))?;
		let mut obj: Object = Vec::new().into();
		self.consume_whitespaces();
		if !self.match_and_advance(&[Token::ObjectEnd]) {
			obj.push(self.object_entry()?);
			self.consume_whitespaces();
			while !self.match_and_advance(&[Token::ObjectEnd]) {
				self.consume(Token::Comma, |parser| parser.error(&format!(
					"Expected comma, got: {:?}",
					parser.get_current()
				)))?;
				self.consume_whitespaces();
				obj.push(self.object_entry()?);
				self.consume_whitespaces();
			}
		}
		Ok(obj.into())
	}

	fn object_entry(&mut self) -> Result<(Expr, Expr), OsmiaError> {
		let key = self.expr()?;
		self.consume_whitespaces();
		self.consume(Token::Colon, |parser| parser.error(&format!(
			"Expected colon, got: {:?}",
			parser.get_current()
		)))?;
		self.consume_whitespaces();
		let value = self.expr()?;
		Ok((key, value))
	}

	fn grouping(&mut self) -> Result<Expr, OsmiaError> {
		self.consume(Token::ParentStart, |parser| parser.error(&format!(
			"Expected start of grouping, got: {:?}",
			parser.get_current()
		)))?;
		self.consume_whitespaces();
		let expr = self.expr()?;
		self.consume_whitespaces();
		self.consume(Token::ParentEnd, |parser| parser.error(&format!(
			"Expected end of grouping, got: {:?}",
			parser.get_current()
		)))?;
		Ok(Grouping::new(expr).into())
	}

	fn identifier(&mut self) -> Result<JsonTreeKey<String>, OsmiaError> {
		let key = match self.advance() {
			Token::Alpha(s) => s.as_str().into(),
			_ => return Err(self.error(&format!(
				"Invalid identifier: {:?}",
				self.get_previous()
			)))
		};
		Ok(key)
	}
}
