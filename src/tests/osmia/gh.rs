use super::*;

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
