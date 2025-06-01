use super::{
	Ctx,
	JsonTree,
	CtxValue
};
use serde::Serialize;
use std::collections::BTreeMap;
use serde::ser::{SerializeStruct};

#[derive(Clone)]
pub struct CallableDumpNode {
	arity: Option<usize>,
	#[cfg(feature = "detailed-dumper")]
	description: Option<String>,
}

impl CallableDumpNode {
	pub fn new(arity: Option<usize>, #[cfg(feature = "detailed-dumper")] description: Option<String>) -> Self {
		Self {
			arity,
			#[cfg(feature = "detailed-dumper")]
			description
		}
	}
}

#[derive(Clone)]
pub enum DumpNode {
	Node(Vec<(String, DumpNode)>),
	Array(Vec<DumpNode>),
	Variable(CtxValue),
	Callable(CallableDumpNode),
}

impl DumpNode {
	fn as_node(self) -> Vec<(String, DumpNode)> {
		match self {
			DumpNode::Node(v) => v,
			_ => panic!("expected node!")
		}
	}
}

impl Serialize for DumpNode {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		let mut state = serializer.serialize_struct("DumpNode", 2)?;
		match self {
			DumpNode::Variable(v) => {
				state.serialize_field("type", "variable")?;
				match v {
					CtxValue::Callable(c) => unreachable!("callables are not dumped as variables: {}", c),
					CtxValue::Int(i) => state.serialize_field("value", i)?,
					CtxValue::Float(e) => state.serialize_field("value", e)?,
					CtxValue::Str(e) => state.serialize_field("value", e)?,
					CtxValue::Bool(e) => state.serialize_field("value", e)?,
					CtxValue::Null => state.serialize_field("value", &())?,
				};
			},
			DumpNode::Node(v) => {
				state.serialize_field("type", "object")?;
				let mut obj = BTreeMap::new();
				for (k, v) in v {
					obj.insert(k, v);
				}
				state.serialize_field("value", &obj)?;
			},
			DumpNode::Array(v) => {
				state.serialize_field("type", "array")?;
				state.serialize_field("value", v)?;
			}
			DumpNode::Callable(c) => {
				state.serialize_field("type", "function")?;
				match c.arity {
					Some(a) => state.serialize_field("arity", &a)?,
					None => state.serialize_field("arity", &())?,
				}
				#[cfg(feature = "detailed-dumper")]
				if let Some(d) = &c.description {
					state.serialize_field("description", &d)?;
				}
			}
		};
		state.end()
	}
}

pub struct CtxJsonDumper {}

impl CtxJsonDumper {
	pub fn dump(ctx: &Ctx) -> String {
		let mut dump: Vec<(String, DumpNode)> = Vec::new();
		for scope in ctx.raw().iter().rev() {
			let scope = CtxJsonDumper::dump_node(scope);
			for e in scope.as_node() {
				dump.push(e);
			}
		}
		let dump = DumpNode::Node(dump);
		serde_json::to_string(&dump).unwrap()
	}

	pub fn dump2str(node: DumpNode) -> String {
		serde_json::to_string(&node).unwrap()
	}

	pub fn dump_node(node: &JsonTree<String, CtxValue>) -> DumpNode {
		match node {
			JsonTree::Value(v) => match v {
				CtxValue::Callable(c) => DumpNode::Callable(CallableDumpNode::new(
					c.arity(),
					#[cfg(feature = "detailed-dumper")] c.description()
				)),
				_ => DumpNode::Variable(v.clone()),
			},
			JsonTree::Array(arr) => DumpNode::Array(arr.iter().map(|e| CtxJsonDumper::dump_node(e)).collect()),
			JsonTree::Object(obj) => DumpNode::Node(obj.iter().map(|(k, v)| (k.clone(), CtxJsonDumper::dump_node(v))).collect()),
		}
	}
}
