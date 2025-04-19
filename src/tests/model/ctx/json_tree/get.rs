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
fn tests_valid(keys: Vec<JsonTreeKey<String>>, expected: JsonTree<String, JsonValue>) {
	let ctx = valid_json();
	match ctx.get(&mut keys.iter()) {
		Ok(value) => assert_eq!(value, &expected),
		Err(e) => panic!("Not able to get the value:\n{:?}", e),
	}
}

#[cfg(test)]
fn tests_invalid(keys: Vec<JsonTreeKey<String>>, expected: JsonTreeError<JsonTreeKey<String>>) {
	let ctx = valid_json();
	assert_eq!(ctx.get(&mut keys.iter()), Err(expected));
}

macro_tests!(
	tests_valid,
	(
		basic_get01,
		vec![JsonTreeKey::Key("foo".to_string())],
		JsonTree::Value(JsonValue::Int(1))
	),
	(
		basic_get02,
		vec![
			JsonTreeKey::Key("quux".to_string()),
			JsonTreeKey::Index(0),
			JsonTreeKey::Key("usr".to_string()),
		],
		JsonTree::Value(JsonValue::String("Marvin".to_string()))
	)
);

macro_tests!(
	tests_invalid,
	(
		key_not_found,
		vec![JsonTreeKey::Key("something_random".to_string())],
		JsonTreeError::KeyNotFound(JsonTreeKey::Key("something_random".to_string()))
	),
	(
		index_in_object,
		vec![JsonTreeKey::Index(0)],
		JsonTreeError::IndexInObject
	),
	(
		array_out_of_bounds,
		vec![
			JsonTreeKey::Key("quux".to_string()),
			JsonTreeKey::Index(123)
		],
		JsonTreeError::ArrayOutOfBounds((123, 1))
	),
	(
		key_in_array,
		vec![
			JsonTreeKey::Key("quux".to_string()),
			JsonTreeKey::Key("0".to_string())
		],
		JsonTreeError::KeyInArray
	),
	(
		access_value_key,
		vec![
			JsonTreeKey::Key("foo".to_string()),
			JsonTreeKey::Key("bar".to_string())
		],
		JsonTreeError::AccessValue(JsonTreeKey::Key("bar".to_string()))
	),
	(
		access_value_index,
		vec![
			JsonTreeKey::Key("foo".to_string()),
			JsonTreeKey::Index(0)
		],
		JsonTreeError::AccessValue(JsonTreeKey::Index(0))
	)
);
