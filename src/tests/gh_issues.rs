use super::*;

macro_tests!(
	test_code,
	(
		identity,
		"Hello, world!",
		None,
		None,
		vec![
			(Ctx::new(), Ok("Hello, world!")),
		],
	),
	(
		basic_test01,
		"Hello, {{name}}!",
		None,
		None,
		vec![
			(Ctx::try_from(r#"{ "name": "world" }"#).unwrap(), Ok("Hello, world!")),
		]
	),
	(
		basic_test02,
		"Hello, {{name}}! A number {{number}}, a boolean {{boolean}} an a null {{null}}.",
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"name": "world",
					"number": 42,
					"boolean": true,
					"null": null
				}
			"#).unwrap(), Ok("Hello, world! A number 42, a boolean true an a null null.")),
		]
	),
	(
		basic_test03,
		"Hello, {{name}}! Single value from lst {{lst[1]}} and an object element {{object.obj.key}}.",
		None,
		None,
		vec![
			(Ctx::try_from(r#"
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
			"#).unwrap(), Ok("Hello, world! Single value from lst 22 and an object element value2.")),
		]
	),
	(
		similar_syntax,
		r#"Hello, {{name}}! The syntax is "Hello, {{"{{name}}!"}}"."#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"{ "name": "world" }"#).unwrap(), Ok(r#"Hello, world! The syntax is "Hello, {{name}}!"."#)),
		]
	),
	(
		comment,
		r#"Hello!{{print "This is a comment"}}"#,
		None,
		None,
		vec![
			(Ctx::new(), Ok("Hello!")),
		]
	),
	(
		complex_value01,
		"Hello, {{name.first}} {{name.last}}!",
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"name": {
						"first": "John",
						"last": "Doe"
					}
				}
			"#).unwrap(), Ok("Hello, John Doe!")),
		]
	),
	(
		complex_value02,
		"This is a complex value: {{obj.obj[0].obj}}",
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"obj": {
						"obj": [
							{
								"obj": "value"
							}
						]
					}
				}
			"#).unwrap(), Ok("This is a complex value: value")),
		]
	),
	(
		foreach01,
		r#"Hello, {{name}}!
		{{for item in items}}
		<a href="{{item.url}}">{{item.name}}</a>
		{{done}}"#,
		None,
		None,
		vec![(
			Ctx::try_from(r#"
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
			"#).unwrap(),
			Ok("Hello, world!\n\t\t<a href=\"https://example01.com\">Example01</a>\n\t\t<a href=\"https://example02.org\">Example02</a>\n")
		)]
	),
	(
		foreach02,
		r#"Hello, {{name}}!
		{{for item in items}}
		<li>Element {{item}}</li>
		{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"name": "world",
					"items": [1, 2, 3]
				}
			"#).unwrap(),
			Ok("Hello, world!\n\t\t<li>Element 1</li>\n\t\t<li>Element 2</li>\n\t\t<li>Element 3</li>\n"))
		]
	),
	(
		foreach03,
		r#"{{for item in items}}<ul>{{for i in item.arr}}<li>{{i}}</li>{{done}}</ul>{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
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
			"#).unwrap(),
			Ok(r#"<ul><li>1</li><li>2</li><li>3</li></ul><ul><li>4</li><li>5</li><li>6</li></ul>"#))
		]
	),
	(
		conditional01,
		"{{user.name}} is {{if user.age == 18}}an adult{{fi}}.",
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"user": {
						"name": "John",
						"age": 18
					}
				}
			"#).unwrap(),
			Ok("John is an adult.")
			),
		]
	),
	(
		conditional02,
		"{{for user in users}}{{if user.age >= 18}}{{user.name}} is an adult.{{fi}}{{done}}",
		None,
		None,
		vec![
			(Ctx::try_from(r#"
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
			"#).unwrap(),
			Ok("John is an adult.Jack is an adult.")
			),
		]
	),
	(
		conditional03,
		r#"
Hey!
{{if true}}
	True!
{{fi}}
{{if false}}
	False!
{{fi}}
{{if true}}
	var: {{var}}
{{fi}}"#.trim(),
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"var": "value"
				}
				"#).unwrap(),
				Ok("Hey!\n\tTrue!\n\tvar: value\n")
			),
		],
	),
	(
		spacing01,
		"
Hello, {{name}}!
{{if age >= 18}}You are an adult{{fi}}.".trim(),
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"name": "John",
					"age": 18
				}
			"#).unwrap(),
			Ok("Hello, John!\nYou are an adult.")),
		]
	),
	(
		not_found_key01,
		"Hello, {{usr.name}}!",
		None,
		None,
		vec![
			(Ctx::try_from(r#"{ "user": { "name": "John" } }"#).unwrap(), Err(vec!["usr", "not", "found"])),
			(Ctx::try_from(r#"{ "usr": { "nombre": "John" } }"#).unwrap(), Err(vec!["name", "not", "found"])),
		]
	),
	(
		not_found_key02,
		"Hello, {{lst[i]}}!",
		None,
		None,
		vec![
			(Ctx::try_from(r#"{ "i": 4, "lst": [1, 2, 3] }"#).unwrap(), Err(vec!["index", "out", "bounds"])),
			(Ctx::try_from(r#"{ "i": 4, "lst": { "a": 1, "b": 2 } }"#).unwrap(), Err(vec!["object", "index"])),
			(Ctx::try_from(r#"{ "i": 0, "lst": { } }"#).unwrap(), Err(vec!["object", "index"])),
		]
	),
	(
		foreach_fail01,
		"Hello, {{name}}!
		{{for item in items}}
			<a href=\"{{item.url}}\">{{item.name}}</a>
		{{done}}",
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"name": "world",
					"items": [
						{
							"url": "https://example01.com",
							"name": "Example01"
						},
						123
					]
				}"#
			).unwrap(),Err(vec!["access", "value", "url"])),
		]
	)
);

// GH issues

macro_tests!(
	test_code,
	(
		gh_03_01,
		r#"start
  start offset
  {{for p in pages}}
    # {{p.name}}
  {{done}}
  end offset
end"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
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
			).unwrap(),
			Ok("start
  start offset
    # foo
    # bar
  end offset
end")
			),
		]
	),
	(
		gh_03_02,
		r#"  {{for p in pages}}{{p.name}}{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
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
			).unwrap(),
			Ok("  foobar")),
		]
	),
	(
		gh_03_03,
		r#"{{for p in pages}}  {{p.name}},{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"pages": [
						{
							"name": "foo"
						},
						{
							"name": "bar"
						}
					]
				}"#).unwrap(),
			Ok("  foo,  bar,"))
		]
	),
	(
		gh_03_04,
		r#"{{for p in pages}}  {{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"pages": [1, 2]
				}"#).unwrap(),
			Ok("    "))
		]
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
		None,
		None,
		vec![
			(Ctx::try_from(r#"
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
				}"#).unwrap(),
			Ok(r#"-->
John is an adult.
Jack is an adult.
		<--"#))
		]
	),
	(
		gh_03_06,
		r#"


{{v}}
		{{v}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"v": "foo"
				}
			"#).unwrap(),
			Ok("\n\n\nfoo\n\t\tfoo"))
		]
	),
	(
		gh_03_07,
		r#"


{{if v == 7}}		{{v}}		{{fi}}		
		{{v}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"v": 7
				}"#).unwrap(),
			Ok("\n\n\n\t\t7\t\t\t\t\n\t\t7"))
		]
	),
	(
		gh_03_08,
		r#"
{{for page in pages}}
  {{if page.disabled}}
  {{fi}}
  # {{page.endpoint}}
{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
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
				}"#).unwrap(),
			Ok("\n  # foo\n  # bar\n  # baz\n"))
		]
	),
	(
		gh_03_09,
		r#"  {{for page in pages}}{{if page.disabled}}{{fi}}
  # {{page.endpoint}}
{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
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
				}"#).unwrap(),
			Ok("  # foo\n  # bar\n  # baz\n"))
		]
	),
	(
		gh_03_10,
		"\n\n\n\n\n    {{v}}",
		None,
		None,
		vec![
			(Ctx::try_from(r#"{"v": "foo"}"#).unwrap(), Ok("\n\n\n\n\n    foo"))
		]
	)
);

macro_tests!(
	test_code,
	(
		gh_01_01,
		r#"
{{for page in pages}}
  {{if page.disabled}}
    {{continue}}
  {{fi}}
  # {{page.endpoint}}
{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
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
				}"#).unwrap(),
			Ok("\n  # bar\n  # baz\n"))
		]
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
		None,
		None,
		vec![
			(Ctx::try_from(r#"
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
				}"#).unwrap(),
			Ok("\n  # bar\n  # baz\n"))
		]
	),
	(
		gh_01_03,
		r#"{{for p in pages}}{{p}}{{break}}{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"{"pages": [1,2,3]}"#).unwrap(), Ok("1")),
		]
	),
	(
		gh_01_04,
		r#"{{if true}}{{for p in pages}}{{p}}{{break}}{{done}}{{fi}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"{"pages": [1,2,3]}"#).unwrap(), Ok("1")),
		]
	),
	(
		gh_01_05,
		r#"{{while true}}{{if true}}{{for p in pages}}{{p}}{{break}}{{done}}{{fi}}{{break}}{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"{"pages": [1,2,3]}"#).unwrap(), Ok("1")),
		]
	),
	(
		gh_01_06,
		r#"{{v = 0}}{{while v < 10}}{{v}}{{v = v + 1}}{{continue}}NO{{done}}"#,
		None,
		None,
		vec![
			(Ctx::new(), Ok("0123456789"))
		]
	),
	(
		gh_01_07,
		r#"{{v = 0}}{{while v < 10}}{{v}}{{v = v + 1}}{{if true}}--{{continue}}{{fi}}NO{{done}}"#,
		None,
		None,
		vec![
			(Ctx::new(), Ok("0--1--2--3--4--5--6--7--8--9--"))
		]
	),
	(
		gh_01_08,
		r#"{{for n in nbrs}}{{n}}{{continue}}{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"{"nbrs": [1,2,3]}"#).unwrap(), Ok("123")),
		]
	),
	(
		gh_01_09,
		r#"{{for n in nbrs}}{{n}}{{if true}}--{{continue}}{{fi}}{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"{"nbrs": [1,2,3]}"#).unwrap(), Ok("1--2--3--")),
		]
	),
	(
		gh_13_01,
		r#"{{ 6 & 3 }} {{ true & true }} {{ true & false }} {{ false & true }} {{ false & false }}"#,
		None,
		None,
		vec![
			(Ctx::new(), Ok("2 true false false false"))
		]
	),
	(
		gh_13_02,
		r#"{{ 6 | 3 }} {{ true | true }} {{ true | false }} {{ false | true }} {{ false | false }}"#,
		None,
		None,
		vec![
			(Ctx::new(), Ok("7 true true true false"))
		]
	),
	(
		gh_13_03,
		r#"{{ 6 ^ 3 }} {{ true ^ true }} {{ true ^ false }} {{ false ^ true }} {{ false ^ false }}"#,
		None,
		None,
		vec![
			(Ctx::new(), Ok("5 false true true false"))
		]
	),
	(
		gh_13_04,
		r#"{{ 6 << 2 }} {{ 6 >> 2 }} {{ 6.0 << 2 }} {{ 6.0 >> 2 }} {{ 6.3 << 2 }} {{ 6.9 >> 2 }}"#,
		None,
		None,
		vec![
			(Ctx::new(), Ok("24 1 24 1 24 1"))
		]
	),
	(
		gh_13_05,
		r#"{{ "Foo" << 2 }} {{ "Foo" >> 2 }}"#,
		None,
		None,
		vec![
			(Ctx::new(), Ok("o F"))
		]
	)
);

macro_tests!(
	test_code,
	(
		gh_16_01,
		r#"{{if !nothing}}Nothing is null{{fi}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"{"nothing": null}"#).unwrap(), Ok("Nothing is null"))
		]
	)
);

macro_tests!(
	test_code,
	(
		gh_17_01,
		r#"
{{if (foo != null) && foo}}
1. foo
{{fi}}
{{if foo != null && foo}}
2. foo
{{fi}}"#.trim(),
		None,
		None,
		vec![
			(Ctx::try_from(r#"{"foo": null }"#).unwrap(), Ok("")),
			(Ctx::try_from(r#"{"foo": false }"#).unwrap(), Ok("")),
			(Ctx::try_from(r#"{"foo": true }"#).unwrap(), Ok("1. foo\n2. foo\n")),
			(Ctx::try_from(r#"{"foo": "Something not null"}"#).unwrap(), Ok("1. foo\n2. foo\n"))
		]
	)
);
