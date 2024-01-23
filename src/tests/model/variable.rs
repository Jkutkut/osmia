use crate::syntax_tree::model::{
	Variable
};

#[test]
fn invalid_json_values() {
	let tests = [
		"",
		" ",
		".",
		".age",
		"user.",
		"user..age",
		"user.age.",
		"user.age..",
		"user.age[",
		"user.age[0",
		"user.age[0.",
		"user.age[0].",
		" user",
		"user ",
		"user.first_name-key",
		"user[af]",
		"[0]",
		"[]",
		"user[0.2]",
		"user[2fs3]",
		"user[0][0.2]",
		"user.[0]",
		"user[[0]",
		"user[0]]",
		"u[.0]",
		"u[0.]",
		"u[0[0]]",
		"u[]",
		"u[0][0]]",
	];
	for test in tests.iter() {
		match Variable::from_str(test) {
			Some(var) => {
				panic!("'{}' should not compile: {:?}", test, var);
			},
			None => assert!(true)
		}
	}
}

#[test]
fn json_value02() {
	let tests = [
		"user",
		"user.age",
		"user.surnames[0]",
		"user.surnames[0].length",
		"user.first_name",
		"this.key.is.really.long.arr[0][120][14560].key",
		"u[1].key[2][3].hola"
	];
	for test in tests.iter() {
		match Variable::from_str(test) {
			Some(var) => {
				println!("{:?}", var);
				assert!(true);
			},
			None => {
				panic!("Failed to parse: {}", test);
			}
		}
	}
}
