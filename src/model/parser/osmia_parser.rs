use super::{
	Parser,
	ParserErrorMsg,
};
use crate::types::{
	LexerCode,
	ParserCode,
	OsmiaError,
};
use crate::model::expr::*;
use crate::model::stmt::*;
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

pub struct OsmiaParserImpl {
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
			return Err(self.error_msg(
				ParserErrorMsg::Custom("Unexpected token".to_string())
			));
		}
		Ok(code)
	}

	fn code(&mut self) -> Result<ParserCode, OsmiaError> {
		self.block()
	}
}

impl OsmiaParserImpl {
	#[cfg(debug_assertions)]
	fn error_msg(&self, msg: ParserErrorMsg) -> String {
		let tokens_until_now = &self.code[0..self.current];
		format!(
			"Parser error: Line {}: {}\nTokens until now: {:?} -> {:?}",
			self.line, msg.report(self), tokens_until_now, self.get_current()
		)
	}

	#[cfg(not(debug_assertions))]
	fn error_msg(&self, msg: ParserErrorMsg) -> String {
		format!(
			"Parser error: Line {}: {}",
			self.line, msg.report(self),
		)
	}

	fn done(&self) -> bool {
		self.check_current(&Token::Eof)
	}

	fn check_current(&self, token: &Token) -> bool {
		self.get_current() == token
	}

	pub fn get_current(&self) -> &Token {
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
				Token::NewLineNonPrintable => self.line += 1,
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
				Token::NewLineNonPrintable => statements.push(Stmt::NewLineNonPrintable),
				Token::Raw(r) => statements.push(Stmt::Raw(r.to_string())),
				Token::NonPrintable(r) => statements.push(Stmt::NonPrintable(r.to_string())),
				Token::StmtStart => match self.stmt(break_with)? {
					None => break,
					Some(stmt) => statements.push(stmt),
				},
				_ => return Err(self.error_msg(
					ParserErrorMsg::Custom("Unexpected in block:".to_string())
				))
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
		self.consume(Token::StmtEnd, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::StmtEnd)
		))?;
		Ok(Some(stmt))
	}

	fn print(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Print, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::Print)
		))?;
		self.consume_whitespaces();
		Ok(Stmt::new_print(self.expr()?))
	}

	fn comment(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Comment, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::Comment)
		))?;
		let mut comment = String::new();
		while !self.done() && !self.check_current(&Token::StmtEnd) {
			match self.advance() {
				Token::Raw(r) | Token::NonPrintable(r) => comment.push_str(r),
				Token::NewLine | Token::NewLineNonPrintable => {
					comment.push('\n');
					self.line += 1;
				},
				_ => return Err(self.error_msg(
					ParserErrorMsg::ParseValue("comment".into())
				))
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
		self.consume(Token::If, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::If)
		))?;
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
			self.consume(Token::StmtEnd, |parser| parser.error_msg(
				ParserErrorMsg::Unclosed("else statement".to_string(), Token::StmtEnd)
			))?;
			else_block = Some(self.breakable_block(Some(&vec![Token::Fi]))?);
		}
		Ok(Stmt::If(If::new(conditional, else_if_blocks, else_block)))
	}

	fn while_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::While, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::While)
		))?;
		Ok(self.conditional(&vec![
			Token::While, Token::Done
		])?.into())
	}

	fn for_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::For, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::For)
		))?;
		self.consume_whitespaces();
		let var = Variable::from_vec(vec![self.identifier()?.into()]);
		self.consume_whitespaces();
		self.consume(Token::In, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::In)
		))?;
		self.consume_whitespaces();
		let iterable = self.expr()?;
		self.consume_whitespaces();
		self.consume(Token::StmtEnd, |parser| parser.error_msg(
			ParserErrorMsg::Unclosed("for statement".to_string(), Token::StmtEnd)
		))?;
		let block = self.breakable_block(Some(&vec![Token::Done]))?;
		Ok(Stmt::For(For::new(var, iterable, block)))
	}

	fn conditional(&mut self, break_with: &Vec<Token>) -> Result<ConditionalStmt, OsmiaError> {
		self.consume_whitespaces();
		let expr = self.expr()?;
		self.consume_whitespaces();
		self.consume(Token::StmtEnd, |parser| parser.error_msg(
			ParserErrorMsg::Unclosed("conditional statement".to_string(), Token::StmtEnd)
		))?;
		let block = self.breakable_block(Some(break_with))?;
		Ok(ConditionalStmt::new(expr, block))
	}

	fn break_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Break, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::Break)
		))?;
		Ok(Stmt::Break)
	}

	fn continue_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Continue, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::Continue)
		))?;
		Ok(Stmt::Continue)
	}

	fn return_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Return, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::Return)
		))?;
		self.consume_whitespaces();
		let expr: Option<Expr> = match self.check_current(&Token::StmtEnd) {
			true => None,
			false => Some(self.expr()?),
		};
		Ok(Stmt::new_return(expr))
	}

	fn function(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume(Token::Function, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::Function)
		))?;
		self.consume_whitespaces();
		let name = self.identifier()?;
		self.consume_whitespaces();
		let params = match self.match_and_advance(&[Token::Semicolon]) {
			true => self.parameters(&Token::StmtEnd)?,
			false => Vec::new(),
		};
		self.consume_whitespaces();
		self.consume(Token::StmtEnd, |parser| parser.error_msg(
			ParserErrorMsg::Unclosed("function statement".to_string(), Token::Arrow)
		))?;
		let block = self.breakable_block(Some(&vec![Token::Done]))?;
		Ok(Stmt::Function(Function::new(name, params, block)))
	}

	fn parameters(&mut self, exit_token: &Token) -> Result<Vec<FunctionParam>, OsmiaError> {
		self.consume_whitespaces();
		let mut are_mandatory_params_allowed = true;
		let mut params: Vec<FunctionParam> = Vec::new();
		while !self.check_current(exit_token) {
			if params.len() > 0 {
				self.consume(Token::Comma, |parser| parser.error_msg(
					ParserErrorMsg::Expected(Token::Comma)
				))?;
				self.consume_whitespaces();
			}
			if self.match_and_advance(&[Token::Spread]) {
				params.push(FunctionParam::new_spread(self.identifier()?));
				self.consume_whitespaces();
				break;
			}
			let param = self.parameter()?;
			match &param {
				FunctionParam::Param(_, None) if !are_mandatory_params_allowed => {
					return Err(format!(
						"Invalid parameter: A mandatory parameter can not be after an optional parameter: {}",
						param
					));
				},
				FunctionParam::Param(_, Some(_)) if are_mandatory_params_allowed => {
					are_mandatory_params_allowed = false;
				},
				_ => (),
			};
			params.push(param);
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
		self.consume(Token::Function, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::Function)
		))?;
		self.consume_whitespaces();
		self.consume(Token::ParentStart, |parser| parser.error_msg(
			ParserErrorMsg::Expected(Token::ParentStart)
		))?;
		let params = self.parameters(&Token::ParentEnd)?;
		self.consume(Token::ParentEnd, |parser| parser.error_msg(
			ParserErrorMsg::Unclosed("lambda".to_string(), Token::ParentEnd)
		))?;
		self.consume_whitespaces();
		self.consume(Token::Arrow, |parser| parser.error_msg(
			ParserErrorMsg::MissingKeyword(Token::Arrow)
		))?;
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
		self.value()
	}

	fn value(&mut self) -> Result<Expr, OsmiaError> {
		let mut value = self.primary()?;
		loop {
			value = match self.get_current() {
				Token::Question => self.method_call(value)?,
				Token::ParentStart => self.call(value)?,
				Token::Dot | Token::ArrayStart => self.variable(value)?,
				_ => break,
			}
		}
		Ok(value)
	}

	fn primary(&mut self) -> Result<Expr, OsmiaError> {
		match self.get_current() {
			Token::ArrayStart => self.array(),
			Token::ObjectStart => self.object(),
			Token::ParentStart => self.grouping(),
			Token::Str(_) | Token::Number(_) | Token::Bool(_) | Token::Null => self.literal(),
			_ => Ok(Variable::from_name(self.identifier()?.into()).into()),
		}
	}

	fn array(&mut self) -> Result<Expr, OsmiaError> {
		self.consume(Token::ArrayStart, |parser| parser.error_msg(
			ParserErrorMsg::Expected(Token::ArrayStart)
		))?;
		let mut arr: Array = Vec::new().into();
		self.consume_whitespaces();
		if !self.match_and_advance(&[Token::ArrayEnd]) {
			arr.push(self.expr()?.into());
			self.consume_whitespaces();
			while !self.match_and_advance(&[Token::ArrayEnd]) {
				self.consume(Token::Comma, |parser| parser.error_msg(
					ParserErrorMsg::Expected(Token::Comma)
				))?;
				self.consume_whitespaces();
				arr.push(self.expr()?.into());
				self.consume_whitespaces();
			}
		}
		Ok(arr.into())
	}

	fn object(&mut self) -> Result<Expr, OsmiaError> {
		self.consume(Token::ObjectStart, |parser| parser.error_msg(
			ParserErrorMsg::Expected(Token::ObjectStart)
		))?;
		let mut obj: Object = Vec::new().into();
		self.consume_whitespaces();
		if !self.match_and_advance(&[Token::ObjectEnd]) {
			obj.push(self.object_entry()?).unwrap();
			self.consume_whitespaces();
			while !self.match_and_advance(&[Token::ObjectEnd]) {
				self.consume(Token::Comma, |parser| parser.error_msg(
					ParserErrorMsg::Expected(Token::Comma)
				))?;
				self.consume_whitespaces();
				obj.push(self.object_entry()?).unwrap();
				self.consume_whitespaces();
			}
		}
		Ok(obj.into())
	}

	fn object_entry(&mut self) -> Result<(Expr, Expr), OsmiaError> {
		let key = self.expr()?;
		self.consume_whitespaces();
		self.consume(Token::Colon, |parser| parser.error_msg(
			ParserErrorMsg::Expected(Token::Colon)
		))?;
		self.consume_whitespaces();
		let value = self.expr()?;
		Ok((key, value))
	}

	fn grouping(&mut self) -> Result<Expr, OsmiaError> {
		self.consume(Token::ParentStart, |parser| parser.error_msg(
			ParserErrorMsg::Expected(Token::ParentStart)
		))?;
		self.consume_whitespaces();
		let expr = self.expr()?;
		self.consume_whitespaces();
		self.consume(Token::ParentEnd, |parser| parser.error_msg(
			ParserErrorMsg::Unclosed("grouping".to_string(), Token::ParentEnd)
		))?;
		Ok(Grouping::new(expr).into())
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
						Err(_) => return Err(self.error_msg(
							ParserErrorMsg::ParseValue("float".into())
						))
					},
					false => match n.parse::<i64>() {
						Ok(i) => Expr::Int(i),
						Err(_) => return Err(self.error_msg(
							ParserErrorMsg::ParseValue("int".into())
						))
					}
				}
			},
			_ => unreachable!(),
		};
		self.advance();
		Ok(expr)
	}

	fn method_call(&mut self, mut obj: Expr) -> Result<Expr, OsmiaError> {
		while self.match_and_advance(&[Token::Question]) {
			let name = Variable::from_name(self.identifier()?.into()).into();
			match self.call(name)? {
				Expr::Call(call) => obj = MethodCall::new(obj, call).into(),
				_ => return Err(self.error_msg(
					ParserErrorMsg::Custom(
						"Expected method call. Maybe you forgot to call it?".to_string()
					)
				))
			};
		}
		Ok(obj)
	}

	fn call(&mut self, mut callable: Expr) -> Result<Expr, OsmiaError> {
		while self.check_current(&Token::ParentStart) {
			callable = Call::new(callable, self.arguments()?).into();
		}
		Ok(callable)
	}

	fn arguments(&mut self) -> Result<Vec<Expr>, OsmiaError> {
		self.consume(Token::ParentStart, |parser| parser.error_msg(
			ParserErrorMsg::Expected(Token::ParentStart)
		))?;
		let mut arr = Vec::new();
		self.consume_whitespaces();
		if !self.check_current(&Token::ParentEnd) {
			arr.push(self.expr()?.into());
			self.consume_whitespaces();
			while !self.check_current(&Token::ParentEnd) {
				self.consume(Token::Comma, |parser| parser.error_msg(
					ParserErrorMsg::Expected(Token::Comma)
				))?;
				self.consume_whitespaces();
				arr.push(self.expr()?.into());
				self.consume_whitespaces();
			}
		}
		self.consume(Token::ParentEnd, |parser| parser.error_msg(
			ParserErrorMsg::Unclosed("arguments".to_string(), Token::ParentEnd)
		))?;
		Ok(arr)
	}

	fn variable(&mut self, name: Expr) -> Result<Expr, OsmiaError> {
		let mut var: Vec<JsonTreeKeyExpr> = vec![];
		match name {
			Expr::Variable(v) => var = v.into(),
			e => var.push(e.into())
		}
		loop {
			match self.get_current() {
				Token::Dot => {
					self.advance();
					var.push(self.identifier()?.into());
				},
				Token::ArrayStart => {
					self.advance();
					var.push(self.expr()?.into());
					self.consume(Token::ArrayEnd, |parser| parser.error_msg(
						ParserErrorMsg::Unclosed("array selector".to_string(), Token::ArrayEnd)
					))?;
				},
				_ => break
			}
		}
		Ok(Variable::from_vec(var).into())
	}

	fn identifier(&mut self) -> Result<JsonTreeKey<String>, OsmiaError> {
		let key = match self.advance() {
			Token::Alpha(s) => s.as_str().into(),
			_ => return Err(self.error_msg(
				ParserErrorMsg::Custom("Invalid identifier:".to_string())
			)),
		};
		Ok(key)
	}
}
