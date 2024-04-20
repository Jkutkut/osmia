use crate::lexer::Token;
use crate::model::{
	Stmt, Block
};
use super::{test_parser};


#[test]
fn basic_test() {
	test_parser(
		vec![Token::Raw("Hello World!".to_string())],
		Stmt::Raw("Hello World!".to_string()),
	);
}

#[test]
fn basic_test2() {
	test_parser(
		vec![
			Token::Raw("Hello World!".to_string()),
			Token::Raw("Hello World!".to_string()),
		],
		Stmt::Block(Block::new(vec![
			Stmt::Raw("Hello World!".to_string()),
			Stmt::Raw("Hello World!".to_string()),
		])),
	);
}
