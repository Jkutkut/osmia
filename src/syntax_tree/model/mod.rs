mod expression;
mod binary;
mod grouping;
mod literal;
mod unary;
mod variable;

mod impl_visitor;

pub use binary::Binary;
pub use expression::Expression;
pub use grouping::Grouping;
pub use literal::Literal;
pub use unary::Unary;
pub use variable::Variable;

enum Stmt<'a> {
	Expression(Expression<'a>),
	// Print(Expression<'a>),
	// Var(&'a str, Option<Expression<'a>>),
	// Block(Vec<Stmt<'a>>),
	If(Expression<'a>, Box<Stmt<'a>>, Option<Box<Stmt<'a>>>),
	// While(Expression<'a>, Box<Stmt<'a>>),
	ForEach(Variable<'a>, Variable<'a>, Box<Stmt<'a>>),
	// Break,
	// Continue,
}

enum CodeBlock<'a> {
	Stmt(Stmt<'a>),
	Raw(&'a str),
}
