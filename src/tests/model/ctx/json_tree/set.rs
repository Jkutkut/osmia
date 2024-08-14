use std::collections::HashMap;
use crate::macro_tests;
use super::{
	valid_json,
	JsonValue
};
use crate::model::ctx::{
	JsonTree,
	JsonTreeError,
	JsonTreeKey,
};

#[cfg(test)]
fn key_in_ctx(keys: Vec<JsonTreeKey<String>>, expected: JsonTree<String, JsonValue>) {
	let mut ctx = valid_json();
	match ctx.set(&mut keys.iter(), expected.clone()) {
		Ok(_) => (),
		Err(e) => panic!("Not able to set the value:\n{:?}", e),
	};
	match ctx.get(&mut keys.iter()) {
		Ok(value) => assert_eq!(value, &expected),
		Err(e) => panic!("Not able to get the value:\n{:?}", e),
	}
}

#[cfg(test)]
fn tests_invalid(
	keys: Vec<JsonTreeKey<String>>,
	value: JsonTree<String, JsonValue>,
	expected: JsonTreeError<JsonTreeKey<String>>
) {
	let mut ctx = valid_json();
	match ctx.set(&mut keys.iter(), value) {
		Ok(_) => panic!("This set should have failed"),
		Err(e) => assert_eq!(e, expected),
	};
}

macro_tests!(
	key_in_ctx,
	(
		basic_set01,
		vec![JsonTreeKey::Key("foofoo".to_string())],
		JsonTree::Value(JsonValue::Int(1))
	),
	(
		replace,
		vec![JsonTreeKey::Key("foo".to_string())],
		JsonTree::Value(JsonValue::Int(2))
	),
	(
		advanced_set01,
		vec![
			JsonTreeKey::Key("quux".to_string()),
			JsonTreeKey::Index(0),
			JsonTreeKey::Key("usr".to_string())
		],
		JsonTree::Value(JsonValue::String("Marvin".to_string()))
	),
	(
		increment_array,
		vec![JsonTreeKey::Key("quux".to_string()), JsonTreeKey::Index(1)],
		JsonTree::Value(JsonValue::Int(2))
	),
	(
		grow_array01,
		vec![JsonTreeKey::Key("quux".to_string()), JsonTreeKey::Index(10)],
		JsonTree::Value(JsonValue::Int(2))
	),
	(
		set_array01,
		vec![JsonTreeKey::Key("array".to_string())],
		JsonTree::Array(vec![
			JsonTree::Value(JsonValue::Int(1)),
			JsonTree::Value(JsonValue::Int(2)),
			JsonTree::Value(JsonValue::Int(3))
		])
	),
	(
		set_object01,
		vec![JsonTreeKey::Key("object".to_string())],
		JsonTree::Object(HashMap::from([
			("foo".to_string(), Box::new(JsonTree::Value(JsonValue::Int(1)))),
			("bar".to_string(), Box::new(JsonTree::Value(JsonValue::Int(2)))),
			("baz".to_string(), Box::new(JsonTree::Value(JsonValue::Int(3))))
		]))
	)
);

macro_tests!(
	tests_invalid,
	(
		access_value01,
		vec![
			JsonTreeKey::Key("foo".to_string()),
			JsonTreeKey::Key("bar".to_string())
		],
		JsonTree::Value(JsonValue::String("Hey!".to_string())),
		JsonTreeError::AccessValue(JsonTreeKey::Key("bar".to_string()))
	),
	(
		access_value02,
		vec![
			JsonTreeKey::Key("foo".to_string()),
			JsonTreeKey::Index(0)
		],
		JsonTree::Value(JsonValue::String("Hey!".to_string())),
		JsonTreeError::AccessValue(JsonTreeKey::Index(0))
	),
	(
		access_value03,
		vec![
			JsonTreeKey::Key("foo".to_string()),
			JsonTreeKey::Key("bar".to_string()),
			JsonTreeKey::Key("baz".to_string())
		],
		JsonTree::Value(JsonValue::String("Hey!".to_string())),
		JsonTreeError::AccessValue(JsonTreeKey::Key("bar".to_string()))
	),
	(
		access_value04,
		vec![
			JsonTreeKey::Key("foo".to_string()),
			JsonTreeKey::Index(0),
			JsonTreeKey::Index(1)
		],
		JsonTree::Value(JsonValue::String("Hey!".to_string())),
		JsonTreeError::AccessValue(JsonTreeKey::Index(0))
	),
	(
		key_in_array01,
		vec![
			JsonTreeKey::Key("quux".to_string()),
			JsonTreeKey::Key("user".to_string())
		],
		JsonTree::Value(JsonValue::String("Marvin".to_string())),
		JsonTreeError::KeyInArray
	),
	(
		key_in_array02,
		vec![
			JsonTreeKey::Key("quux".to_string()),
			JsonTreeKey::Key("user".to_string()),
			JsonTreeKey::Key("usr".to_string())
		],
		JsonTree::Value(JsonValue::String("Marvin".to_string())),
		JsonTreeError::KeyInArray
	),
	(
		index_in_object01,
		vec![
			JsonTreeKey::Key("quux".to_string()),
			JsonTreeKey::Index(0),
			JsonTreeKey::Index(1)
		],
		JsonTree::Value(JsonValue::Int(2)),
		JsonTreeError::IndexInObject
	),
	(
		index_in_object02,
		vec![
			JsonTreeKey::Index(0),
			JsonTreeKey::Index(1)
		],
		JsonTree::Value(JsonValue::Int(2)),
		JsonTreeError::IndexInObject
	),
	(
		array_out_of_bounds01,
		vec![
			JsonTreeKey::Key("quux".to_string()),
			JsonTreeKey::Index(1000),
			JsonTreeKey::Key("usr".to_string())
		],
		JsonTree::Value(JsonValue::Int(2)),
		JsonTreeError::ArrayOutOfBounds((1000, 1))
	),
	(
		key_not_found01,
		vec![
			JsonTreeKey::Key("nonexistent".to_string()),
			JsonTreeKey::Key("key".to_string())
		],
		JsonTree::Value(JsonValue::Int(1)),
		JsonTreeError::KeyNotFound(JsonTreeKey::Key("nonexistent".to_string()))
	)
);
