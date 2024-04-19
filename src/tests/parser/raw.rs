use crate::lexer::Token;
use crate::model::{
	Stmt, Block
};
use super::{test_parser};


#[test]
fn basic_test() {
	test_parser(
		vec![Token::Raw("Hello World!")],
		Stmt::Raw("Hello World!"),
	);
}

#[test]
fn basic_test2() {
	test_parser(
		vec![
			Token::Raw("Hello World!"),
			Token::Raw("Hello World!"),
		],
		Stmt::Block(Block::new(vec![
			Stmt::Raw("Hello World!"),
			Stmt::Raw("Hello World!"),
		])),
	);
}
