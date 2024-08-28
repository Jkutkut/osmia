use super::*;

// GH issues
macro_tests!(
	test,
	(
		gh_01_01,
		Some(r#"
{{for page in pages}}
  {{if page.disabled}}
    {{continue}}
  {{fi}}
  # {{page.endpoint}}
{{done}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"pages": [
		// 			{
		// 				"disabled": true,
		// 				"endpoint": "foo"
		// 			},
		// 			{
		// 				"disabled": false,
		// 				"endpoint": "bar"
		// 			},
		// 			{
		// 				"disabled": false,
		// 				"endpoint": "baz"
		// 			}
		// 		]
		// 	}
		// "#),
		// "\n  # bar\n  # baz\n"
	),
	(
		gh_01_02,
		Some(r#"
{{for page in pages}}
  {{if true}}
  {{if page.disabled}}
    {{continue}}
  {{fi}}
  {{fi}}
  # {{page.endpoint}}
{{done}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"pages": [
		// 			{
		// 				"disabled": true,
		// 				"endpoint": "foo"
		// 			},
		// 			{
		// 				"disabled": false,
		// 				"endpoint": "bar"
		// 			},
		// 			{
		// 				"disabled": false,
		// 				"endpoint": "baz"
		// 			}
		// 		]
		// 	}
		// "#),
		// "\n  # bar\n  # baz\n"
	),
	(
		gh_01_03,
		Some(r#"{{for p in pages}}{{p}}{{break}}{{done}}"#),
		None,
		None,
		None
		// Some(r#"{"pages": [1,2,3]}"#),
		// "1"
	),
	(
		gh_01_04,
		Some(r#"{{if true}}{{for p in pages}}{{p}}{{break}}{{done}}{{fi}}"#),
		None,
		None,
		None
		// Some(r#"{"pages": [1,2,3]}"#),
		// "1"
	),
	(
		gh_01_05,
		Some(r#"{{while true}}{{if true}}{{for p in pages}}{{p}}{{break}}{{done}}{{fi}}{{break}}{{done}}"#),
		None,
		None,
		None
		// Some(r#"{"pages": [1,2,3]}"#),
		// "1"
	),
	(
		gh_01_06,
		Some(r#"{{assign v = 0}}{{while v < 10}}{{v}}{{assign v = v + 1}}{{continue}}NO{{done}}"#),
		None,
		None,
		None
		// None,
		// "0123456789"
	),
	(
		gh_01_07,
		Some(r#"{{assign v = 0}}{{while v < 10}}{{v}}{{assign v = v + 1}}{{if true}}--{{continue}}{{fi}}NO{{done}}"#),
		None,
		None,
		None
		// None,
		// "0--1--2--3--4--5--6--7--8--9--"
	),
	(
		gh_01_08,
		Some(r#"{{for n in nbrs}}{{n}}{{continue}}{{done}}"#),
		None,
		None,
		None
		// Some(r#"{"nbrs": [1,2,3]}"#),
		// "123"
	),
	(
		gh_01_09,
		Some(r#"{{for n in nbrs}}{{n}}{{if true}}--{{continue}}{{fi}}{{done}}"#),
		None,
		None,
		None
		// Some(r#"{"nbrs": [1,2,3]}"#),
		// "1--2--3--"
	),
	(
		gh_13_01,
		Some(r#"{{ 6 & 3 }} {{ true & true }} {{ true & false }} {{ false & true }} {{ false & false }} {{ null & null }}"#),
		None,
		None,
		None
		// None,
		// "2 true false false false null"
	),
	(
		gh_13_02,
		Some(r#"{{ 6 | 3 }} {{ true | true }} {{ true | false }} {{ false | true }} {{ false | false }} {{ null | null }}"#),
		None,
		None,
		None
		// None,
		// "7 true true true false null"
	),
	(
		gh_13_03,
		Some(r#"{{ 6 ^ 3 }} {{ true ^ true }} {{ true ^ false }} {{ false ^ true }} {{ false ^ false }}"#),
		None,
		None,
		None
		// None,
		// "5 false true true false"
	),
	(
		gh_13_04,
		Some(r#"{{ 6 << 2 }} {{ 6 >> 2 }} {{ 6.0 << 2 }} {{ 6.0 >> 2 }} {{ 6.3 << 2 }} {{ 6.9 >> 2 }}"#),
		None,
		None,
		None
		// None,
		// "24 1 24 1 24 1"
	),
	(
		gh_13_05,
		Some(r#"{{ "Foo" << 2 }} {{ "Foo" >> 2 }}"#),
		None,
		None,
		None
		// None,
		// "o F"
	)
);

macro_tests!(
	test,
	(
		gh_03_01,
		Some(r#"start
  start offset
  {{for p in pages}}
    # {{p.name}}
  {{done}}
  end offset
end"#),
		None,
		None,
		None
		/*Some(r#"
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
end"*/
	),
	(
		gh_03_02,
		Some(r#"  {{for p in pages}}{{p.name}}{{done}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"pages": [
		// 			{
		// 				"name": "foo"
		// 			},
		// 			{
		// 				"name": "bar"
		// 			}
		// 		]
		// 	}
		// "#),
		// "  foobar"
	),
	(
		gh_03_03,
		Some(r#"{{for p in pages}}  {{p.name}},{{done}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"pages": [
		// 			{
		// 				"name": "foo"
		// 			},
		// 			{
		// 				"name": "bar"
		// 			}
		// 		]
		// 	}
		// "#),
		// "  foo,  bar,"
	),
	(
		gh_03_04,
		Some(r#"{{for p in pages}}  {{done}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"pages": [1, 2]
		// 	}
		// "#),
		// "    "
	),
	(
		gh_03_05,
		Some(r#"-->
		{{for user in users}}
			{{if user.age >= 18}}
{{user.name}} is an adult.
			{{fi}}
		{{done}}
		<--"#),
		None,
		None,
		None
		/*Some(r#"
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
		<--"#*/
	),
	(
		gh_03_06,
		Some(r#"


{{v}}
		{{v}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"v": "foo"
		// 	}
		// "#),
		// "\n\n\nfoo\n\t\tfoo"
	),
	(
		gh_03_07,
		Some(r#"


{{if v == 7}}		{{v}}		{{fi}}		
		{{v}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"v": 7
		// 	}
		// "#),
		// "\n\n\n\t\t7\t\t\t\t\n\t\t7"
	),
	(
		gh_03_08,
		Some(r#"
{{for page in pages}}
  {{if page.disabled}}
  {{fi}}
  # {{page.endpoint}}
{{done}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"pages": [
		// 			{
		// 				"disabled": true,
		// 				"endpoint": "foo"
		// 			},
		// 			{
		// 				"disabled": false,
		// 				"endpoint": "bar"
		// 			},
		// 			{
		// 				"disabled": false,
		// 				"endpoint": "baz"
		// 			}
		// 		]
		// 	}
		// "#),
		// "\n  # foo\n  # bar\n  # baz\n"
	),
	(
		gh_03_09,
		Some(r#"  {{for page in pages}}{{if page.disabled}}{{fi}}
  # {{page.endpoint}}
{{done}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"pages": [
		// 			{
		// 				"disabled": true,
		// 				"endpoint": "foo"
		// 			},
		// 			{
		// 				"disabled": false,
		// 				"endpoint": "bar"
		// 			},
		// 			{
		// 				"disabled": false,
		// 				"endpoint": "baz"
		// 			}
		// 		]
		// 	}
		// "#),
		// "  # foo\n  # bar\n  # baz\n"
	),
	(
		gh_03_10,
		Some("\n\n\n\n\n    {{v}}"),
		None,
		None,
		None
		// Some(r#"{"v": "foo"}"#),
		// "\n\n\n\n\n    foo"
	)
);

macro_tests!(
	test,
	(
		gh_16_01,
		Some(r#"{{if !nothing}}Nothing is null{{fi}}"#),
		None,
		None,
		None
		// Some(r#"{"nothing": null}"#),
		// "Nothing is null"
	)
);

macro_tests!(
	test,
	(
		gh_17_01,
		Some(r#"
{{if (foo != null) && foo}}
1. foo does exists
{{fi}}
{{if foo != null && foo}}
2. foo does exists
{{fi}}"#.trim()),
		None,
		None,
		None
		// None,
		// ""
	),
	(
		gh_17_02,
		Some(r#"
{{if (foo != null) && foo}}
1. foo does exists
{{fi}}
{{if foo != null && foo}}
2. foo does exists
{{fi}}"#.trim()),
		None,
		None,
		None
		// Some(r#"{"foo": "Something not null"}"#),
		// "1. foo does exists\n2. foo does exists\n"
	)
);
