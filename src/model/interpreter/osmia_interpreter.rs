use std::cell::RefCell;

use crate::types::*;
use super::Interpreter;

pub struct OsmiaInterpreter<'ctx> {
	#[allow(dead_code)]
	ctx: RefCell<&'ctx mut Ctx>,
}


impl<'ctx> OsmiaInterpreter<'ctx> {
	pub fn new(ctx: &'ctx mut Ctx) -> Self {
		Self {
			ctx: RefCell::new(ctx),
		}
	}
}

impl Interpreter<ParserCode, OsmiaOutput, OsmiaError> for OsmiaInterpreter<'_> {
	#[allow(unused_variables)]
	fn interpret(&self, code: ParserCode) -> Result<OsmiaOutput, OsmiaError> {
		todo!() // TODO
		// Interpret code
	}
}
