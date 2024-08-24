use super::*;

macro_tests!(
	lexer_test,
	(empty,"",vec![]),
	(raw_whitespace_01," ",vec![Token::new_raw(" ")]),
	(raw_whitespace_02," \t",vec![Token::new_raw(" \t")]),
	(raw_newline_01,"\n",vec![Token::NewLine]),
	(raw_newline_02,"\n\n\n",vec![Token::NewLine,Token::NewLine,Token::NewLine]),
	(raw_newline_03,"\nFoo\nBar\n",vec![
		Token::NewLine,
		Token::new_raw("Foo"),
		Token::NewLine,
		Token::new_raw("Bar"),
		Token::NewLine
	]),
	(raw_text,"Hey there! This is a raw text chunk",vec![Token::new_raw("Hey there! This is a raw text chunk")]),
	(stmt_start_01,"{{}}",vec![Token::StmtStart,Token::StmtEnd]),
	(stmt_start_02,"\n{{}}",vec![Token::NewLine,Token::StmtStart,Token::StmtEnd]),
	(stmt_start_03,"{{\n\n\t      \n  }}",vec![Token::StmtStart,Token::NewLine,Token::NewLine,Token::NewLine,Token::StmtEnd]),
	(stmt_start_04,"{{}}\n",vec![Token::StmtStart,Token::StmtEnd,Token::NewLine]),
	(stmt_start_05,"This is a {{}} block with {{}}{{}} {{}} multiple blocks",vec![
		Token::new_raw("This is a "),
		Token::StmtStart, Token::StmtEnd,
		Token::new_raw(" block with "),
		Token::StmtStart, Token::StmtEnd,
		Token::StmtStart, Token::StmtEnd,
		Token::new_raw(" "),
		Token::StmtStart, Token::StmtEnd,
		Token::new_raw(" multiple blocks")
	]),
);

macro_tests!(
	lexer_test_fail,
	(invalid_stmt_start_01,"{{\n\n\t      \n  ", "}}"),
	(invalid_stmt_start_02,"{{ {", "}}"),
	(invalid_stmt_start_03,"{{ }", "}}"),
);
