use super::*;

macro_tests!(
	interpreter_test,
	(
		sort,
		r#"{{ a?sort() }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"] }"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "a"]}"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": [3, 2, 1] }"#).unwrap(), Ok(r#"[1, 2, 3]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 3] }"#).unwrap(), Ok(r#"[1, 2, 3]"#)),
			(Ctx::try_from(r#"{ "a": [1, 3, 2] }"#).unwrap(), Ok(r#"[1, 2, 3]"#)),
			(Ctx::try_from(r#"{ "a": [true, false, null, 2, 2.3, "str"] }"#).unwrap(), Ok(r#"[false, true, null, 2, 2.3, "str"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", true, false, null, 2, 2.3, "str"] }"#).unwrap(), Ok(r#"[false, "a", true, null, 2, 2.3, "str"]"#)),
		]
	),
	(
		sort_by,
		r#"{{ a?sort_by(fn (a, b) => a - b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [2, 1] }"#).unwrap(), Ok(r#"[1, 2]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2] }"#).unwrap(), Ok(r#"[1, 2]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 4, 3] }"#).unwrap(), Ok(r#"[1, 2, 3, 4]"#)),
		]
	),
	(
		sort_by02,
		r#"{{ a?sort_by(fn (a, b) => b - a) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [2, 1] }"#).unwrap(), Ok(r#"[2, 1]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2] }"#).unwrap(), Ok(r#"[2, 1]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 4, 3] }"#).unwrap(), Ok(r#"[4, 3, 2, 1]"#)),
			(Ctx::try_from(r#"{ "a": [null, 2] }"#).unwrap(), Ok(r#"[null, 2]"#)),
		]
	),
	(
		// Sort alphabetically except if the first character is 'c'
		sort_by03,
		r#"{{ a?sort_by(fn (a, b) => (a?starts_with("c"))?then(
			(b?starts_with("c"))?then((a < b)?then(1, -1), 1),
			(b?starts_with("c"))?then(-1, (b < a)?then(1, -1))
		)) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"] }"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "a"]}"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "a", "b"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "c", "b"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "b", "a"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "c", "a"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["hey", "call", "foo"] }"#).unwrap(), Ok(r#"["foo", "hey", "call"]"#)),
		]
	),
	(
		sort_by04,
		r#"{{ a?sort_by(fn (a, b) => a < b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [2, 1] }"#).unwrap(), Ok(r#"[1, 2]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2] }"#).unwrap(), Ok(r#"[1, 2]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 4, 3] }"#).unwrap(), Ok(r#"[1, 2, 3, 4]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"] }"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "a"]}"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "a", "b"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "c", "b"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "b", "a"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "c", "a"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["hey", "call", "foo"] }"#).unwrap(), Ok(r#"["call", "foo", "hey"]"#)),
		]
	),
	(
		sort_by05,
		r#"{{ a?sort_by(fn (a, b) => a > b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [2, 1] }"#).unwrap(), Ok(r#"[2, 1]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2] }"#).unwrap(), Ok(r#"[2, 1]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 4, 3] }"#).unwrap(), Ok(r#"[4, 3, 2, 1]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"] }"#).unwrap(), Ok(r#"["b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "a"]}"#).unwrap(), Ok(r#"["b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "a", "b"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "c", "b"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "b", "a"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "c", "a"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["hey", "call", "foo"] }"#).unwrap(), Ok(r#"["hey", "foo", "call"]"#)),
		]
	),
	(
		map01,
		r#"{{ a?map(fn (a) => a?upper()) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["foo"] }"#).unwrap(), Ok(r#"["FOO"]"#)),
			(Ctx::try_from(r#"{ "a": ["foo", "bar"] }"#).unwrap(), Ok(r#"["FOO", "BAR"]"#)),
		]
	),
	(
		map02,
		r#"{{ a?map(fn (a) => a * 2) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [1] }"#).unwrap(), Ok(r#"[2]"#)),
			(Ctx::try_from(r#"{ "a": [1.2, 6] }"#).unwrap(), Ok(r#"[2.4, 12]"#)),
		]
	),
	(
		map03,
		r#"{{ a?map(fn (a) => a?has_content()) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [null, false, true, "", "hola", 0, 1, 2] }"#).unwrap(), Ok(r#"[false, true, true, false, true, true, true, true]"#)),
		]
	),
	(
		for_each01,
		r#"{{fn increment; e}}{{s = s + e}}{{done}}{{s = ""}}{{arr?for_each(fn (e) => increment(e)) }}{{s}}"#,
		vec![
			(Ctx::try_from(r#"{ "arr": [] }"#).unwrap(), Ok(r#""#)),
			(Ctx::try_from(r#"{ "arr": ["a", "b", "c"] }"#).unwrap(), Ok(r#"abc"#)),
		]
	),
	(
		for_each_index01,
		r#"{{fn add_even; e, idx}}{{if idx % 2 == 0}}{{s = s + e}}{{fi}}{{done}}{{s = ""}}{{arr?for_each_index(fn (e, idx) => add_even(e, idx)) }}{{s}}"#,
		vec![
			(Ctx::try_from(r#"{ "arr": [] }"#).unwrap(), Ok(r#""#)),
			(Ctx::try_from(r#"{ "arr": ["a", "b", "c"] }"#).unwrap(), Ok(r#"ac"#)),
		]
	),
	(
		reverse,
		r#"{{ a?reverse() }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["a"] }"#).unwrap(), Ok(r#"["a"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"] }"#).unwrap(), Ok(r#"["b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
		]
	),
	(
		filter01,
		r#"{{ a?filter(fn (a) => a?has_content()) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [null, false, true, "", "hola", 0, 1, 2] }"#).unwrap(), Ok(r#"[false, true, "hola", 0, 1, 2]"#)),
		]
	),
	(
		filter_02,
		r#"{{ a?filter(fn (a) => a?to_bool()) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [null, false, true, "", "hola", 0, 1, 2] }"#).unwrap(), Ok(r#"[true, "hola", 1, 2]"#)),
		]
	),
	(
		filter_03,
		r#"{{ a?filter(fn (a) => b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [], "b": false }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [], "b": true}"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [null, false, true, "", "hola", 0, 1, 2], "b": false }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [null, false, true, "", "hola", 0, 1, 2], "b": true }"#).unwrap(), Ok(r#"[null, false, true, "", "hola", 0, 1, 2]"#)),
		]
	),
	(
		filter_index01,
		r#"{{ a?filter_index(fn (a, idx) => idx % 2 == 0) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"["a", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c", "d"] }"#).unwrap(), Ok(r#"["a", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c", "d", "e"] }"#).unwrap(), Ok(r#"["a", "c", "e"]"#)),
		]
	),
	(
		filter_index02,
		r#"{{ a?filter_index(fn (a, idx) => false) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"[]"#)),
		]
	),
	(
		filter_index03,
		r#"{{ a?filter_index(fn (a, idx) => true) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
		]
	),
	(
		reduce01,
		r#"{{ a?reduce(fn (a, b) => a + b, 0) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Err(vec!["Reduce", "empty", "array"])),
			(Ctx::try_from(r#"{ "a": [3] }"#).unwrap(), Ok(r#"3"#)),
			(Ctx::try_from(r#"{ "a": [1, 2] }"#).unwrap(), Ok(r#"3"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 3] }"#).unwrap(), Ok(r#"6"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 3, 4] }"#).unwrap(), Ok(r#"10"#)),
		]
	),
	(
		reduce02,
		r#"{{ a?reduce(fn (a, b) => false, null) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Err(vec!["Reduce", "empty", "array"])),
			(Ctx::try_from(r#"{ "a": ["a"] }"#).unwrap(), Ok(r#"false"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"] }"#).unwrap(), Ok(r#"false"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"false"#)),
		]
	),
	(
		join,
		r#"{{ a?join(j) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [], "j": "" }"#).unwrap(), Ok(r#""#)),
			(Ctx::try_from(r#"{ "a": ["a"], "j": "" }"#).unwrap(), Ok(r#"a"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"], "j": "" }"#).unwrap(), Ok(r#"ab"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"], "j": "" }"#).unwrap(), Ok(r#"abc"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"], "j": "-" }"#).unwrap(), Ok(r#"a-b-c"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"], "j": ", " }"#).unwrap(), Ok(r#"a, b, c"#)),
			(Ctx::try_from(r#"{ "a": [null, true, false, []], "j": ", " }"#).unwrap(), Ok(r#"null, true, false, []"#)),
		]
	),
	(
		keys,
		r#"{{ a?keys() }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"[0, 1, 2]"#)),
			(Ctx::try_from(r#"{ "a": [null, true, false, []] }"#).unwrap(), Ok(r#"[0, 1, 2, 3]"#)),
		]
	),
	(
		values,
		r#"{{ a?values() }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": [null, true, false, []] }"#).unwrap(), Ok(r#"[null, true, false, []]"#)),
		]
	),
	(
		entries,
		r#"[{{ a?entries()?map(fn (e) => e.key + ":" + e.value)?join(",") }}]"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [null] }"#).unwrap(), Ok(r#"[0:null]"#)),
			(Ctx::try_from(r#"{ "a": [true, false, []] }"#).unwrap(), Ok(r#"[0:true,1:false,2:[]]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"[0:a,1:b,2:c]"#)),
		]
	),
	(
		get,
		r#"{{ a?get(k, d) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [], "k": 0, "d": "?" }"#).unwrap(), Ok(r#"?"#)),
			(Ctx::try_from(r#"{ "a": ["a"], "k": 0, "d": "?" }"#).unwrap(), Ok(r#"a"#)),
			(Ctx::try_from(r#"{ "a": ["a"], "k": 1, "d": "?" }"#).unwrap(), Ok(r#"?"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"], "k": 0, "d": "?" }"#).unwrap(), Ok(r#"a"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"], "k": 1, "d": "?" }"#).unwrap(), Ok(r#"b"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"], "k": 2, "d": "?" }"#).unwrap(), Ok(r#"?"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"], "k": -1, "d": "?" }"#).unwrap(), Err(vec!["Invalid", "index"])),
			(Ctx::try_from(r#"{ "a": ["a", "b"], "k": "0", "d": "?" }"#).unwrap(), Err(vec!["Invalid", "index"])),
			(Ctx::try_from(r#"{ "a": ["a", "b"], "k": true, "d": "?" }"#).unwrap(), Err(vec!["Invalid", "index"])),
		]
	)
);
