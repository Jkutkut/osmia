use crate::macro_tests;
use crate::Osmia;

#[cfg(test)]
fn test_io(
	input: &str,
	ctx: Option<&str>,
	expected: &str
) {
	println!("Code:\n{:?}", input);
	let mut interpreter = match ctx {
		None => Osmia::new(),
		Some(ctx) => Osmia::from_json(ctx).unwrap()
	};
	let code = Osmia::code(input).unwrap();
	let result = interpreter.run(&code).unwrap();
	assert_eq!(result, expected);
}

#[cfg(test)]
fn test_execution_error(
	input: &str,
	ctx: Option<&str>
) {
	let mut interpreter = match ctx {
		None => Osmia::new(),
		Some(ctx) => Osmia::from_json(ctx).unwrap()
	};
	let code = Osmia::code(input).unwrap();
	let result = interpreter.run(&code);
	assert!(result.is_err());
}

macro_tests!(
	test_io,
	(
		identity,
		"Hello, world!",
		None,
		"Hello, world!"
	),
	(
		basic_test01,
		"Hello, {{name}}!",
		Some(r#"{ "name": "world" }"#),
		"Hello, world!"
	),
	(
		basic_test02,
		"Hello, {{name}}! A number {{number}}, a boolean {{boolean}} an a null {{null}}.",
		Some(r#"
			{
				"name": "world",
				"number": 42,
				"boolean": true,
				"null": null
			}
		"#),
		"Hello, world! A number 42, a boolean true an a null null."
	),
	(
		basic_test03,
		"Hello, {{name}}! Single value from lst {{lst[1]}} and an object element {{object.obj.key}}.",
		Some(r#"
			{
				"name": "world",
				"lst": [11, 22, 33],
				"object": {
					"key": "value",
					"obj": {
						"key": "value2"
					}
				}
			}
		"#),
		"Hello, world! Single value from lst 22 and an object element value2."
	),
	(
		similar_syntax,
		r#"Hello, {{name}}! The syntax is "Hello, {{"{{name}}!"}}"."#,
		Some(r#"{ "name": "world" }"#),
		"Hello, world! The syntax is \"Hello, {{name}}!\"."
	),
	(
		comment,
		r#"Hello!{{print "This is a comment"}}"#,
		None,
		"Hello!"
	),
	(
		complex_value01,
		"Hello, {{name.first}} {{name.last}}!",
		Some(r#"
			{
				"name": {
					"first": "John",
					"last": "Doe"
				}
			}
		"#),
		"Hello, John Doe!"
	),
	(
		complex_value02,
		"This is a complex value: {{obj.obj[0].obj}}",
		Some(r#"
			{
				"obj": {
					"obj": [
						{
							"obj": "value"
						}
					]
				}
			}
		"#),
		"This is a complex value: value"
	),
	(
		foreach01,
		r#"Hello, {{name}}!
		{{for item in items}}
		<a href="{{item.url}}">{{item.name}}</a>
		{{done}}"#,
		Some(r#"
			{
				"name": "world",
				"items": [
					{
						"url": "https://example01.com",
						"name": "Example01"
					},
					{
						"url": "https://example02.org",
						"name": "Example02"
					}
				]
			}
		"#),
		"Hello, world!
		<a href=\"https://example01.com\">Example01</a>
		<a href=\"https://example02.org\">Example02</a>\n"
	),
	(
		foreach02,
		r#"Hello, {{name}}!
		{{for item in items}}
		<li>Element {{item}}</li>
		{{done}}"#,
		Some(r#"
			{
				"name": "world",
				"items": [1, 2, 3]
			}
		"#),
		"Hello, world!
		<li>Element 1</li>
		<li>Element 2</li>
		<li>Element 3</li>\n"
	),
	(
		foreach03,
		r#"{{for item in items}}<ul>{{for i in item.arr}}<li>{{i}}</li>{{done}}</ul>{{done}}"#,
		Some(r#"
			{
				"items": [
					{
						"arr": [1, 2, 3]
					},
					{
						"arr": [4, 5, 6]
					}
				]
			}
		"#),
		"<ul><li>1</li><li>2</li><li>3</li></ul><ul><li>4</li><li>5</li><li>6</li></ul>"
	),
	(
		conditional01,
		"{{user.name}} is {{if user.age == 18}}an adult{{fi}}.",
		Some(r#"
			{
				"user": {
					"name": "John",
					"age": 18
				}
			}
		"#),
		"John is an adult."
	),
	(
		conditional02,
		"{{for user in users}}{{if user.age >= 18}}{{user.name}} is an adult.{{fi}}{{done}}",
		Some(r#"
			{
				"users": [
					{
						"name": "John",
						"age": 18
					},
					{
						"name": "Jane",
						"age": 17
					},
					{
						"name": "Jack",
						"age": 19
					}
				]
			}
		"#),
		"John is an adult.Jack is an adult."
	),
	(
		conditional03,
		r#"Hey!
		{{if true}}
			True!
		{{fi}}
		{{if false}}
			False!
		{{fi}}
		{{if true}}
			var: {{var}}
		{{fi}}"#,
		Some(r#"
			{
				"var": "value"
			}
		"#),
		"Hey!
			True!
			var: value\n"
	),
	(
		spacing01,
		"Hello, {{name}}!
		{{if age >= 18}}You are an adult{{fi}}.",
		Some(r#"
			{
				"name": "John",
				"age": 18
			}
		"#),
		"Hello, John!
		You are an adult."
	)
);

macro_tests!(
	test_execution_error,
	(
		not_found_key01,
		"Hello, {{usr.name}}!",
		Some(r#"{ "user": { "name": "John" } }"#)
	),
	(
		not_found_key02,
		"Hello, {{lst[4]}}!",
		Some(r#"{ "lst": [1, 2, 3] }"#)
	),
	(
		not_found_key03,
		"Hello, {{lst[4]}}!",
		Some(r#"{ "lst": { "a": 1, "b": 2 } }"#)
	),
	(
		not_found_key04,
		"Hello, {{obj[0]}}!",
		Some(r#"{ "obj": {}}"#)
	),
	(
		foreach_fail01,
		"Hello, {{name}}!
		{{for item in items}}
			<a href=\"{{item.url}}\">{{item.name}}</a>
		{{done}}",
		Some(r#"
			{
				"name": "world",
				"items": [
					{
						"url": "https://example01.com",
						"name": "Example01"
					},
					123
				]
			}
		"#)
	)
);

// GH issues

macro_tests!(
	test_io,
	(
		gh_03_01,
		r#"start
  start offset
  {{for p in pages}}
    # {{p.name}}
  {{done}}
  end offset
end"#,
		Some(r#"
			{
				"pages": [
					{
						"name": "foo"
					},
					{
						"name": "bar"
					}
				]
			}"#
		),
		"start
  start offset
    # foo
    # bar
  end offset
end"
	),
	(
		gh_03_02,
		r#"  {{for p in pages}}{{p.name}}{{done}}"#,
		Some(r#"
			{
				"pages": [
					{
						"name": "foo"
					},
					{
						"name": "bar"
					}
				]
			}
		"#),
		"  foobar"
	),
	(
		gh_03_03,
		r#"{{for p in pages}}  {{p.name}},{{done}}"#,
		Some(r#"
			{
				"pages": [
					{
						"name": "foo"
					},
					{
						"name": "bar"
					}
				]
			}
		"#),
		"  foo,  bar,"
	),
	(
		gh_03_04,
		r#"{{for p in pages}}  {{done}}"#,
		Some(r#"
			{
				"pages": [1, 2]
			}
		"#),
		"    "
	),
	(
		gh_03_05,
		r#"-->
		{{for user in users}}
			{{if user.age >= 18}}
{{user.name}} is an adult.
			{{fi}}
		{{done}}
		<--"#,
		Some(r#"
			{
				"users": [
					{
						"name": "John",
						"age": 18
					},
					{
						"name": "Jane",
						"age": 17
					},
					{
						"name": "Jack",
						"age": 19
					}
				]
			}
		"#),
		r#"-->
John is an adult.
Jack is an adult.
		<--"#
	),
	(
		gh_03_06,
		r#"


{{v}}
		{{v}}"#,
		Some(r#"
			{
				"v": "foo"
			}
		"#),
		"\n\n\nfoo\n\t\tfoo"
	),
	(
		gh_03_07,
		r#"


{{if v == 7}}		{{v}}		{{fi}}		
		{{v}}"#,
		Some(r#"
			{
				"v": 7
			}
		"#),
		"\n\n\n\t\t7\t\t\t\t\n\t\t7"
	),
	(
		gh_03_08,
		r#"
{{for page in pages}}
  {{if page.disabled}}
  {{fi}}
  # {{page.endpoint}}
{{done}}"#,
		Some(r#"
			{
				"pages": [
					{
						"disabled": true,
						"endpoint": "foo"
					},
					{
						"disabled": false,
						"endpoint": "bar"
					},
					{
						"disabled": false,
						"endpoint": "baz"
					}
				]
			}
		"#),
		"\n  # foo\n  # bar\n  # baz\n"
	),
	(
		gh_03_09,
		r#"  {{for page in pages}}{{if page.disabled}}{{fi}}
  # {{page.endpoint}}
{{done}}"#,
		Some(r#"
			{
				"pages": [
					{
						"disabled": true,
						"endpoint": "foo"
					},
					{
						"disabled": false,
						"endpoint": "bar"
					},
					{
						"disabled": false,
						"endpoint": "baz"
					}
				]
			}
		"#),
		"  # foo\n  # bar\n  # baz\n"
	),
	(
		gh_03_10,
		"\n\n\n\n\n    {{v}}",
		Some(r#"{"v": "foo"}"#),
		"\n\n\n\n\n    foo"
	)
);

macro_tests!(
	test_io,
	(
		gh_01_01,
		r#"
{{for page in pages}}
  {{if page.disabled}}
    {{continue}}
  {{fi}}
  # {{page.endpoint}}
{{done}}"#,
		Some(r#"
			{
				"pages": [
					{
						"disabled": true,
						"endpoint": "foo"
					},
					{
						"disabled": false,
						"endpoint": "bar"
					},
					{
						"disabled": false,
						"endpoint": "baz"
					}
				]
			}
		"#),
		"\n  # bar\n  # baz\n"
	),
	(
		gh_01_02,
		r#"
{{for page in pages}}
  {{if true}}
  {{if page.disabled}}
    {{continue}}
  {{fi}}
  {{fi}}
  # {{page.endpoint}}
{{done}}"#,
		Some(r#"
			{
				"pages": [
					{
						"disabled": true,
						"endpoint": "foo"
					},
					{
						"disabled": false,
						"endpoint": "bar"
					},
					{
						"disabled": false,
						"endpoint": "baz"
					}
				]
			}
		"#),
		"\n  # bar\n  # baz\n"
	),
	(
		gh_01_03,
		r#"{{for p in pages}}{{p}}{{break}}{{done}}"#,
		Some(r#"{"pages": [1,2,3]}"#),
		"1"
	),
	(
		gh_01_04,
		r#"{{if true}}{{for p in pages}}{{p}}{{break}}{{done}}{{fi}}"#,
		Some(r#"{"pages": [1,2,3]}"#),
		"1"
	),
	(
		gh_01_05,
		r#"{{while true}}{{if true}}{{for p in pages}}{{p}}{{break}}{{done}}{{fi}}{{break}}{{done}}"#,
		Some(r#"{"pages": [1,2,3]}"#),
		"1"
	),
	(
		gh_01_06,
		r#"{{assign v = 0}}{{while v < 10}}{{v}}{{assign v = v + 1}}{{continue}}NO{{done}}"#,
		None,
		"0123456789"
	),
	(
		gh_01_07,
		r#"{{assign v = 0}}{{while v < 10}}{{v}}{{assign v = v + 1}}{{if true}}--{{continue}}{{fi}}NO{{done}}"#,
		None,
		"0--1--2--3--4--5--6--7--8--9--"
	),
	(
		gh_01_08,
		r#"{{for n in nbrs}}{{n}}{{continue}}{{done}}"#,
		Some(r#"{"nbrs": [1,2,3]}"#),
		"123"
	),
	(
		gh_01_09,
		r#"{{for n in nbrs}}{{n}}{{if true}}--{{continue}}{{fi}}{{done}}"#,
		Some(r#"{"nbrs": [1,2,3]}"#),
		"1--2--3--"
	),
	(
		gh_13_01,
		r#"{{ 6 & 3 }} {{ true & true }} {{ true & false }} {{ false & true }} {{ false & false }} {{ null & null }}"#,
		None,
		"2 true false false false null"
	),
	(
		gh_13_02,
		r#"{{ 6 | 3 }} {{ true | true }} {{ true | false }} {{ false | true }} {{ false | false }} {{ null | null }}"#,
		None,
		"7 true true true false null"
	),
	(
		gh_13_03,
		r#"{{ 6 ^ 3 }} {{ true ^ true }} {{ true ^ false }} {{ false ^ true }} {{ false ^ false }}"#,
		None,
		"5 false true true false"
	),
	(
		gh_13_04,
		r#"{{ 6 << 2 }} {{ 6 >> 2 }} {{ 6.0 << 2 }} {{ 6.0 >> 2 }} {{ 6.3 << 2 }} {{ 6.9 >> 2 }}"#,
		None,
		"24 1 24 1 24 1"
	),
	(
		gh_13_05,
		r#"{{ "Foo" << 2 }} {{ "Foo" >> 2 }}"#,
		None,
		"o F"
	)
);

macro_tests!(
	test_io,
	(
		gh_16_01,
		r#"{{if !nothing}}Nothing is null{{fi}}"#,
		Some(r#"{"nothing": null}"#),
		"Nothing is null"
	)
);

macro_tests!(
	test_io,
	(
		gh_17_01,
		r#"
{{if (foo != null) && foo}}
1. foo does exists
{{fi}}
{{if foo != null && foo}}
2. foo does exists
{{fi}}"#.trim(),
		None,
		""
	),
	(
		gh_17_02,
		r#"
{{if (foo != null) && foo}}
1. foo does exists
{{fi}}
{{if foo != null && foo}}
2. foo does exists
{{fi}}"#.trim(),
		Some(r#"{"foo": "Something not null"}"#),
		"1. foo does exists\n2. foo does exists\n"
	)
);
