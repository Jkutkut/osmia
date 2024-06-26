use crate::macro_tests;
use super::{test_interpreter_basic, expect_error};

macro_tests!(
	test_interpreter_basic,
	(
		static_template,
		"hello world!",
		"hello world!"
	),
	(
		int01,
		"{{ 1 + 2 }} {{ 1 - 2 }} {{ 3 * 5 }} {{ 3 / 5 }} {{ 3 % 5 }} {{ 7 && 5 }} {{ 7 || 5 }}",
		"3 -1 15 0 3 true true"
	),
	(
		int_bool_02,
		"{{ 5 && 0 }} {{ 0 && 5 }} {{ 5 || 0 }} {{ 0 || 5 }}",
		"false false true true"
	),
	(
		float01,
		"{{ 1.2 + 2.0 }} {{ 1.2 - 2.0 }} {{ 3.2 * 5.2 }} {{ 3.2 / 4.2 }} {{ 3.2 % 5.0 }} {{ 7.2 && 5.0 }} {{ 7.0 || 5.2 }}",
		"3.2 -0.8 16.64 0.7619047619047619 3.2 true true"
	),
	(
		float_bool_02,
		"{{ 5.0 && 0.0 }} {{ 0.0 && 5.0 }} {{ 5.0 || 0.0 }} {{ 0.0 || 5.0 }}",
		"false false true true"
	),
	(
		int02,
		"{{ 3 > 5 }} {{ 3 >= 5 }} {{ 3 < 5 }} {{ 3 <= 5 }} {{ 3 == 5 }} {{ 3 != 5 }}",
		"false false true true false true"
	),
	(
		float02,
		"{{ 3.0 > 5.0 }} {{ 3.0 >= 5.0 }} {{ 3.0 < 5.0 }} {{ 3.0 <= 5.0 }} {{ 3.0 == 5.0 }} {{ 3.0 != 5.0 }}",
		"false false true true false true"
	),
	(
		int_float01,
		"{{ 1 + 2.0 }} {{ 1.0 + 2 }} {{ 1 - 2.0 }} {{ 1.0 - 2 }} {{ 1 * 2.0 }} {{ 1.0 * 2 }} {{ 1 / 2.0 }} {{ 1.0 / 2 }}",
		"3 3 -1 -1 2 2 0.5 0.5"
	),
	(
		division,
		"{{ 1 / 2 }} {{ 1.0 / 2 }} {{ 1 / 2.0 }} {{ 1.0 / 2.0 }}",
		"0 0.5 0.5 0.5"
	),
	(
		bool01,
		"{{ true && true }} {{ true && false }} {{ false && true }} {{ false && false }}",
		"true false false false"
	),
	(
		bool02,
		"{{ true || true }} {{ true || false }} {{ false || true }} {{ false || false }}",
		"true true true false"
	),
	(
		str01,
		r#"{{ "hello" + " world" }},{{"null: " + null}},{{"true: " + true}},{{"false: " + false}},{{"int: " + 1}},{{"float: " + 1.1}}"#,
		"hello world,null: null,true: true,false: false,int: 1,float: 1.1"
	),
	(
		str02,
		r#"{{null + "null"}},{{true + "true"}},{{false + "false"}},{{1 + "int"}},{{1.6 + "float"}}"#,
		"nullnull,truetrue,falsefalse,1int,1.6float"
	),
	(
		unary01,
		"{{ !true }} {{ !false }} {{ !!true }} {{ !!false }}",
		"false true true false"
	),
	(
		unary02,
		"{{ -+-3 }} {{-2}}",
		"3 -2"
	),
	(
		bool03,
		"{{ 1 + true }} {{ 1 + false }} {{ 0 + true }} {{ 0 + false }} {{ 3 + true }} {{ 3 + false }}",
		"true true true false true true"
	),
	(
		bool04,
		"{{ 1 - true }} {{ 1 - false }} {{ 0 - true }} {{ 0 - false }} {{ 3 - true }} {{ 3 - false }}",
		"true false false true true false"
	),
	(
		unary03,
		"{{ !!3 }} {{ !!-3 }} {{ !!0 }}",
		"true true false"
	),
	(
		grouping01,
		"{{ (1 + 2) * 3 }}",
		"9"
	),
	(
		grouping02,
		"{{ 1 + (2 * 3) }}",
		"7"
	),
	(
		grouping03,
		"{{ (1 + 2) * (3 + 4) }}",
		"21"
	),
	(
		float_infinity,
		"{{ 1.0 / 0.0 }}",
		"inf"
	),
	(
		float_neg_infinity,
		"{{ -1.0 / 0.0 }}",
		"-inf"
	)
);

macro_tests!(
	test_interpreter_basic,
	(basic_operation00, "2 + 1 = {{2 + 1}}", "2 + 1 = 3"),
	(basic_operation01, "2 - 1 = {{2 - 1}}", "2 - 1 = 1"),
	(basic_operation02, "2 * 3 = {{2 * 3}}", "2 * 3 = 6"),
	(basic_operation03, "6 / 2 = {{6 / 2}}", "6 / 2 = 3"),
	(basic_operation04, "6 % 4 = {{6 % 4}}", "6 % 4 = 2"),
	(basic_operation05, "2 + 3 * 4 = {{2 + 3 * 4}}", "2 + 3 * 4 = 14"),
	(basic_operation06, "(2 + 3) * 4 = {{(2 + 3) * 4}}", "(2 + 3) * 4 = 20"),
	(basic_operation07, "2 + 3 * 4 - 5 = {{2 + 3 * 4 - 5}}", "2 + 3 * 4 - 5 = 9"),
	(basic_operation08, "2 + 3 * (4 - 5) = {{2 + 3 * (4 - 5)}}", "2 + 3 * (4 - 5) = -1"),
	(basic_operation09, r#"{{ 3 & 1 }}"#, r#"1"#),
	(basic_operation10, r#"{{ 3 | 1 }}"#, r#"3"#),
	(basic_operation11, r#"{{ 3 ^ 1 }}"#, r#"2"#),
	(basic_operation12, r#"{{ 1 << 2 }}"#, r#"4"#),
	(basic_operation13, r#"{{ 4 >> 2 }}"#, r#"1"#),
	// Decimals
	(basic_operation14_1, "7 / 2 = {{7 / 2}}", "7 / 2 = 3"),
	(basic_operation15_2, "7 / 2 = {{7.0 / 2}}", "7 / 2 = 3.5"),
	(basic_operation16, "7 + 3.2 - 1.2 = {{7 + 3.2 - 1.2}}", "7 + 3.2 - 1.2 = 9"),
	(basic_operation17, "7.0 / 2.0 = {{7.0 / 2.0}}", "7.0 / 2.0 = 3.5"),
	// String
	(basic_str_op01, r#"{{ "abc" + "def" }}"#, "abcdef"),
	(basic_str_op02, r#"{{ "abc" == "def" }}"#, r#"false"#),
	(basic_str_op03, r#"{{ "abc" != "def" }}"#, r#"true"#),
	(basic_str_op04, r#"{{ "abc" < "def" }}"#, r#"true"#),
	(basic_str_op05, r#"{{ "abc" > "def" }}"#, r#"false"#),
	(basic_str_op06, r#"{{ "abc" <= "def" }}"#, r#"true"#),
	(basic_str_op07, r#"{{ "abc" >= "def" }}"#, r#"false"#),
	(basic_str_op08, r#"{{ "abc" == "abc" }}"#, r#"true"#),
	(basic_str_op09, r#"{{ "abc" != "abc" }}"#, r#"false"#),
	(basic_str_op10, r#"{{ "abc" < "abc" }}"#, r#"false"#),
	(basic_str_op11, r#"{{ "abc" > "abc" }}"#, r#"false"#),
	(basic_str_op12, r#"{{ "abc" <= "abc" }}"#, r#"true"#),
	(basic_str_op13, r#"{{ "abc" >= "abc" }}"#, r#"true"#)
);

macro_tests!(
	expect_error,
	(
		int_infinity,
		"{{ 1 / 0 }}",
		r#"{}"#
	),
	(
		int_neg_infinity,
		"{{ -1 / 0 }}",
		r#"{}"#
	),
	(
		invalid_div01,
		"{{ 1 / 0 }}",
		r#"{}"#
	),
	(
		overflow01,
		"{{ 9223372036854775807 + 1 }}",
		r#"{}"#
	),
	(
		overflow02,
		"{{ -9223372036854775807 - 2 }}",
		r#"{}"#
	),
	(
		overflow03,
		"{{ 9223372036854775807 * 2 }}",
		r#"{}"#
	),
	(
		overflow04,
		"{{ -9223372036854775807 * 2 }}",
		r#"{}"#
	)
);
