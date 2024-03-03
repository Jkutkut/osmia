use crate::lexer::Token;
use crate::model::{
	Expression, Literal, Binary, Grouping, Unary,
	JsonExpression, ListOrVariable,
	Stmt, Variable, ForEach, If, Block, ConditionalBlock, Assign
};
use crate::tree_walker::SyntaxTreePrinter;
use crate::syntax_tree::Visitable;
use crate::macro_tests;

#[cfg(test)]
fn test_visit(
	expected: &str,
	expression: Expression<'_>
) {
	let actual = expression.accept(&SyntaxTreePrinter);
	assert_eq!(actual, expected);
}

fn test_stmt(
	expected: &str,
	expression: Stmt<'_>
) {
	let actual = expression.accept(&SyntaxTreePrinter);
	assert_eq!(actual, expected);
}

macro_tests!(
	test_visit,
	(
		test_str,
		"Hello, world!",
		Expression::Literal(Literal::Str(String::from("Hello, world!")))
	),
	(
		test_int,
		"42",
		Expression::Literal(Literal::Int(42))
	),
	(
		test_float,
		"42.5",
		Expression::Literal(Literal::Float(42.5))
	),
	(
		test_null,
		"null",
		Expression::Literal(Literal::Null)
	),
	(
		test_true,
		"true",
		Expression::Literal(Literal::Bool(true))
	),
	(
		test_false,
		"false",
		Expression::Literal(Literal::Bool(false))
	),
	(
		test_expr01,
		"(5 + 3) * 2 <= !!true",
		Expression::Binary(Binary::new(
			Expression::Grouping(Grouping::new(
				Expression::Binary(Binary::new(
					Expression::Literal(Literal::Int(5)),
					Token::Plus,
					Expression::Literal(Literal::Int(3))
				).unwrap())
			)),
			Token::Multiply,
			Expression::Binary(Binary::new(
				Expression::Literal(Literal::Int(2)),
				Token::LessEqual,
				Expression::Unary(Unary::new(
					Token::Not,
					Expression::Unary(Unary::new(
						Token::Not,
						Expression::Literal(Literal::Bool(true))
					).unwrap())
				).unwrap())
			).unwrap())
		).unwrap())
	)
);

macro_tests!(
	test_stmt,
	(
		test_foreach,
		"for ( x in y ) {  }",
		Stmt::ForEach(ForEach::new(
			Variable::from_str("x").unwrap(),
			ListOrVariable::Variable(Variable::from_str("y").unwrap()),
			Stmt::Block(Block::new(Vec::new()))
		))
	),
	(
		test_foreach_list,
		"for ( x in [1, 2, 3] ) {  }",
		Stmt::ForEach(ForEach::new(
			Variable::from_str("x").unwrap(),
			ListOrVariable::List(JsonExpression::Array(vec![
				JsonExpression::Expression(Expression::Literal(Literal::Int(1))),
				JsonExpression::Expression(Expression::Literal(Literal::Int(2))),
				JsonExpression::Expression(Expression::Literal(Literal::Int(3))),
			])),
			Stmt::Block(Block::new(Vec::new()))
		))
	),
	(
		test_if,
		"if ( x ) {  }",
		Stmt::If(If::new(
			ConditionalBlock::new(
				Expression::Variable(Variable::from_str("x").unwrap()),
				Stmt::Block(Block::new(Vec::new()))
			),
			None,
			None
		))
	),
	(
		test_if_else,
		"if ( x ) {  } else {  }",
		Stmt::If(If::new(
			ConditionalBlock::new(
				Expression::Variable(Variable::from_str("x").unwrap()),
				Stmt::Block(Block::new(Vec::new()))
			),
			None,
			Some(Stmt::Block(Block::new(Vec::new())))
		))
	),
	(
		test_if_else_if,
		"if ( x ) {  } else if ( y ) {  } else {  }",
		Stmt::If(If::new(
			ConditionalBlock::new(
				Expression::Variable(Variable::from_str("x").unwrap()),
				Stmt::Block(Block::new(Vec::new()))
			),
			Some(vec!(
				ConditionalBlock::new(
					Expression::Variable(Variable::from_str("y").unwrap()),
					Stmt::Block(Block::new(Vec::new()))
				)
			)),
			Some(Stmt::Block(Block::new(Vec::new())))
		))
	),
	(
		test_raw,
		"Hello, world!",
		Stmt::Raw("Hello, world!")
	),
	(
		test_print,
		"print x",
		Stmt::Print(
			Expression::Variable(Variable::from_str("x").unwrap())
		)
	),
	(
		test_assign,
		"x = y",
		Stmt::Assign(
			Assign::new(
				Variable::from_str("x").unwrap(),
				JsonExpression::Expression(Expression::Variable(Variable::from_str("y").unwrap()))
			)
		)
	),
	(
		test_while,
		"while ( x ) {  }",
		Stmt::While(
			ConditionalBlock::new(
				Expression::Variable(Variable::from_str("x").unwrap()),
				Stmt::Block(Block::new(Vec::new()))
			)
		)
	),
	(
		test_loop_manipulation,
		"while ( true ) { break }; if ( false ) { continue }",
		Stmt::Block(Block::new(vec!(
			Stmt::While(
				ConditionalBlock::new(
					Expression::Literal(Literal::Bool(true)),
					Stmt::Break
				)
			),
			Stmt::If(If::new(
				ConditionalBlock::new(
					Expression::Literal(Literal::Bool(false)),
					Stmt::Continue
				),
				None,
				None
			))
		)))
	),
	(
		test_expr02,
		"!!x * (23 - (-1 / 2)) == true && false",
		Stmt::Expression(Expression::Binary(Binary::new(
			Expression::Binary(Binary::new(
				Expression::Unary(Unary::new(
					Token::Not,
					Expression::Unary(Unary::new(
						Token::Not,
						Expression::Variable(Variable::from_str("x").unwrap())
					).unwrap())
				).unwrap()),
				Token::Multiply,
				Expression::Grouping(Grouping::new(
					Expression::Binary(Binary::new(
						Expression::Literal(Literal::Int(23)),
						Token::Minus,
						Expression::Grouping(Grouping::new(
							Expression::Binary(Binary::new(
								Expression::Unary(Unary::new(
									Token::Minus,
									Expression::Literal(Literal::Int(1))
								).unwrap()),
								Token::Divide,
								Expression::Literal(Literal::Int(2))
							).unwrap())
						))
					).unwrap())
				))
			).unwrap()),
			Token::Equal,
			Expression::Binary(Binary::new(
				Expression::Literal(Literal::Bool(true)),
				Token::And,
				Expression::Literal(Literal::Bool(false))
			).unwrap())
		).unwrap()))
	),
	(
		test_expr03,
		"true && false || true",
		Stmt::Expression(Expression::Binary(Binary::new(
			Expression::Binary(Binary::new(
				Expression::Literal(Literal::Bool(true)),
				Token::And,
				Expression::Literal(Literal::Bool(false))
			).unwrap()),
			Token::Or,
			Expression::Literal(Literal::Bool(true))
		).unwrap()))
	),
	(
		test_expr04,
		"true || false && true",
		Stmt::Expression(Expression::Binary(Binary::new(
			Expression::Literal(Literal::Bool(true)),
			Token::Or,
			Expression::Binary(Binary::new(
				Expression::Literal(Literal::Bool(false)),
				Token::And,
				Expression::Literal(Literal::Bool(true))
			).unwrap())
		).unwrap()))
	),
	(
		test_expr05,
		"12 == 12 <= 12 < 13 > 12 >= 12 != 10",
		Stmt::Expression(Expression::Binary(Binary::new(
			Expression::Binary(Binary::new(
				Expression::Literal(Literal::Int(12)),
				Token::Equal,
				Expression::Literal(Literal::Int(12))
			).unwrap()),
			Token::LessEqual,
			Expression::Binary(Binary::new(
				Expression::Literal(Literal::Int(12)),
				Token::LessThan,
				Expression::Binary(Binary::new(
					Expression::Literal(Literal::Int(13)),
					Token::GreaterThan,
					Expression::Binary(Binary::new(
						Expression::Literal(Literal::Int(12)),
						Token::GreaterEqual,
						Expression::Binary(Binary::new(
							Expression::Literal(Literal::Int(12)),
							Token::NotEqual,
							Expression::Literal(Literal::Int(10))
						).unwrap())
					).unwrap())
				).unwrap())
			).unwrap())
		).unwrap()))
	)
);
