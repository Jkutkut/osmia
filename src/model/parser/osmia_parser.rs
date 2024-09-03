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
	#[cfg(not(debug_assertions))]
	fn error(&self, msg: &str) -> String {
		format!(
			"Parser error: Line {}: {}",
			self.line, msg
		)
	}

	#[cfg(debug_assertions)]
	fn error(&self, msg: &str) -> String {
		let tokens_until_now = &self.code[0..self.current];
		format!(
			"Parser error: Line {}: {}\nTokens until now: {:?} -> {:?}",
			self.line, msg, tokens_until_now, self.get_current()
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
		self.breakable_block(None)
	}

	fn breakable_block(&mut self, break_with: Option<&Vec<Token>>) -> Result<Stmt, OsmiaError> {
		let mut statements: Block = Block::new();
		while !self.done() {
			match self.advance() {
				Token::NewLine => statements.push(Stmt::NewLine),
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

	fn stmt(&mut self, return_none_with: Option<&Vec<Token>>) -> Result<Option<Stmt>, OsmiaError> {
		if let Some(break_blocks) = return_none_with {
			let is_end_token = self.check_current(&Token::Fi) || self.check_current(&Token::Done);
			for token in break_blocks {
				if self.check_current(token) {
					if is_end_token {
						self.advance();
					}
					return Ok(None)
				}
			}
		}
		let stmt: Stmt = match self.get_current() {
			Token::Print => self.print()?,
			Token::Comment => self.comment()?,
			Token::If => self.if_stmt()?,
			Token::While => self.while_stmt()?,
			Token::For => self.for_stmt()?,
			Token::Break => self.break_stmt()?,
			Token::Continue => self.continue_stmt()?,
			Token::Return => self.return_stmt()?,
			Token::Function => self.function()?,
			_ => self.assign()?,
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

	fn print(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Print, |parser| parser.error(&format!(
			"Expected print, got '{:?}'",
			parser.get_current()
		)))?;
		self.consume_whitespaces();
		Ok(Stmt::new_print(self.expr()?))
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

	fn assign(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume_whitespaces();
		let expr = self.expr()?;
		self.consume_whitespaces();
		if let Expr::Variable(var) = expr {
			return match self.match_and_advance(&[Token::Assign]) {
				true => {
					self.consume_whitespaces();
					Ok(Stmt::new_assign(var, self.expr()?))
				},
				false => Ok(Expr::Variable(var).into())
			}
		}
		Ok(expr.into())
	}

	fn if_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::If, |parser| parser.error(&format!(
			"Expected if, got '{:?}'",
			parser.get_current()
		)))?;
		let conditional = self.conditional(&vec![
			Token::ElseIf, Token::Else, Token::Fi
		])?;
		let mut else_if_blocks = Vec::new();
		while self.match_and_advance(&[Token::ElseIf]) {
			else_if_blocks.push(self.conditional(&vec![
				Token::ElseIf, Token::Else, Token::Fi
			])?);
		}
		let else_if_blocks = match else_if_blocks.len() {
			0 => None,
			_ => Some(else_if_blocks),
		};
		let mut else_block = None;
		if self.match_and_advance(&[Token::Else]) {
			self.consume(Token::StmtEnd, |parser| parser.error(&format!(
				"Unclosed '{:?}' statement. Expected '{:?}' but got '{:?}'",
				Token::Else, Token::StmtEnd, parser.get_current()
			)))?;
			else_block = Some(self.breakable_block(Some(&vec![Token::Fi]))?);
		}
		Ok(Stmt::If(If::new(conditional, else_if_blocks, else_block)))
	}

	fn while_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::While, |parser| parser.error(&format!(
			"Expected while, got '{:?}'",
			parser.get_current()
		)))?;
		Ok(self.conditional(&vec![
			Token::While, Token::Done
		])?.into())
	}

	fn for_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::For, |parser| parser.error(&format!(
			"Expected '{:?}', got '{:?}'",
			Token::For, parser.get_current()
		)))?;
		self.consume_whitespaces();
		let var = Variable::from_vec(vec![self.identifier()?.into()]);
		self.consume_whitespaces();
		self.consume(Token::In, |parser| parser.error(&format!(
			"Expected '{:?}' in for statement, got '{:?}'",
			Token::In, parser.get_current()
		)))?;
		self.consume_whitespaces();
		let iterable = self.expr()?;
		self.consume_whitespaces();
		self.consume(Token::StmtEnd, |parser| parser.error(&format!(
			"Unclosed for statement. Expected '{:?}' but got '{:?}'",
			Token::StmtEnd, parser.get_current()
		)))?;
		let block = self.breakable_block(Some(&vec![Token::Done]))?;
		Ok(Stmt::For(For::new(var, iterable, block)))
	}

	fn conditional(&mut self, break_with: &Vec<Token>) -> Result<ConditionalStmt, OsmiaError> {
		self.consume_whitespaces();
		let expr = self.expr()?;
		self.consume_whitespaces();
		self.consume(Token::StmtEnd, |parser| parser.error(&format!(
			"Unclosed conditional statement. Expected '{:?}' but got '{:?}'",
			Token::StmtEnd, parser.get_current()
		)))?;
		let block = self.breakable_block(Some(break_with))?;
		Ok(ConditionalStmt::new(expr, block))
	}

	fn break_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Break, |parser| parser.error(&format!(
			"Expected break, got '{:?}'",
			parser.get_current()
		)))?;
		Ok(Stmt::Break)
	}

	fn continue_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Continue, |parser| parser.error(&format!(
			"Expected continue, got '{:?}'",
			parser.get_current()
		)))?;
		Ok(Stmt::Continue)
	}

	fn return_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Return, |parser| parser.error(&format!(
			"Expected return, got '{:?}'",
			parser.get_current()
		)))?;
		self.consume_whitespaces();
		let expr: Option<Expr> = match self.check_current(&Token::StmtEnd) {
			true => None,
			false => Some(self.expr()?),
		};
		Ok(Stmt::new_return(expr))
	}

	fn function(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Function, |parser| parser.error(&format!(
			"Expected '{:?}', got '{:?}'",
			Token::Function, parser.get_current()
		)))?;
		self.consume_whitespaces();
		let name = self.identifier()?;
		self.consume_whitespaces();
		let params = match self.match_and_advance(&[Token::Semicolon]) {
			true => self.parameters(&Token::StmtEnd)?,
			false => Vec::new(),
		};
		self.consume_whitespaces();
		self.consume(Token::StmtEnd, |parser| parser.error(&format!(
			"Unclosed function statement. Expected '{:?}' but got '{:?}'",
			Token::StmtEnd, parser.get_current()
		)))?;
		let block = self.breakable_block(Some(&vec![Token::Done]))?;
		Ok(Stmt::Function(Function::new(name, params, block)))
	}

	fn parameters(&mut self, exit_token: &Token) -> Result<Vec<FunctionParam>, OsmiaError> {
		self.consume_whitespaces();
		let mut params: Vec<FunctionParam> = Vec::new();
		while !self.check_current(exit_token) {
			if params.len() > 0 {
				self.consume(Token::Comma, |parser| parser.error(&format!(
					"Expected comma, got: {:?}",
					parser.get_current()
				)))?;
				self.consume_whitespaces();
			}
			if self.match_and_advance(&[Token::Spread]) {
				params.push(FunctionParam::new_spread(self.identifier()?));
				self.consume_whitespaces();
				break;
			}
			params.push(self.parameter()?);
			self.consume_whitespaces();
		}
		Ok(params)
	}

	fn parameter(&mut self) -> Result<FunctionParam, OsmiaError> {
		let name = self.identifier()?;
		self.consume_whitespaces();
		let mut expr: Option<Expr> = None;
		if self.match_and_advance(&[Token::Assign]) {
			self.consume_whitespaces();
			expr = Some(self.expr()?);
		}
		Ok(FunctionParam::new(name, expr))
	}

	fn expr(&mut self) -> Result<Expr, OsmiaError> {
		match self.get_current() {
			Token::Function => self.lambda(),
			_ => self.logic_or()
		}
	}

	fn lambda(&mut self) -> Result<Expr, OsmiaError> {
		self.consume(Token::Function, |parser| parser.error(&format!(
			"Expected lambda keyword '{:?}', got '{:?}'",
			Token::Function, parser.get_current()
		)))?;
		self.consume_whitespaces();
		self.consume(Token::ParentStart, |parser| parser.error(&format!(
			"Expected '{:?}', got '{:?}'",
			Token::ParentStart, parser.get_current()
		)))?;
		let params = self.parameters(&Token::ParentEnd)?;
		self.consume(Token::ParentEnd, |parser| parser.error(&format!(
			"Expected '{:?}', got '{:?}'",
			Token::ParentEnd, parser.get_current()
		)))?;
		self.consume_whitespaces();
		self.consume(Token::Arrow, |parser| parser.error(&format!(
			"Expected '{:?}', got '{:?}'",
			Token::Arrow, parser.get_current()
		)))?;
		self.consume_whitespaces();
		let body = self.expr()?;
		Ok(Lambda::new(params, body).into())
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
		let mut p: Expr = self.primary()?;
		while self.match_and_advance(&[Token::Question]) {
			match self.call()? {
				Expr::Call(call) => p = MethodCall::new(p, call).into(),
				t => return Err(self.error(&format!(
					"Expected method call, got: {:?}", t
				)))?
			};
		}
		Ok(p)
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
		let mut v: Expr = self.variable()?;
		while self.check_current(&Token::ParentStart) {
			v = Call::new(v, self.arguments()?).into();
		}
		Ok(v)
	}

	fn arguments(&mut self) -> Result<Vec<Expr>, OsmiaError> {
		self.consume(Token::ParentStart, |parser| parser.error(&format!(
			"Expected start of arguments, got: {:?}",
			parser.get_current()
		)))?;
		let mut arr = Vec::new();
		self.consume_whitespaces();
		if !self.check_current(&Token::ParentEnd) {
			arr.push(self.expr()?.into());
			self.consume_whitespaces();
			while !self.check_current(&Token::ParentEnd) {
				self.consume(Token::Comma, |parser| parser.error(&format!(
					"Expected comma, got: {:?}",
					parser.get_current()
				)))?;
				self.consume_whitespaces();
				arr.push(self.expr()?.into());
				self.consume_whitespaces();
			}
		}
		self.consume(Token::ParentEnd, |parser| parser.error(&format!(
			"Expected end of arguments, got: {:?}",
			parser.get_current()
		)))?;
		Ok(arr)
	}

	fn variable(&mut self) -> Result<Expr, OsmiaError> {
		Ok(self.obj()?.into())
	}

	fn obj(&mut self) -> Result<Variable, OsmiaError> {
		let mut var = self.arr()?;
		while self.match_and_advance(&[Token::Dot]) {
			var.extend(self.arr()?.into())
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
