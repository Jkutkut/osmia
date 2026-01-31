use super::*;

macro_tests! {
	interpreter_test,
	(
		ctx_get_yaml_01,
		"{{ name }}",
		vec![
			(
				Ctx::try_from_yaml(r#"name: Marvin"#).unwrap(),
				Ok("Marvin")
			),
			(
				Ctx::try_from_yaml(r#"name: "Marvin""#).unwrap(),
				Ok("Marvin")
			),
		]
	),
	(
		ctx_get_yaml_02,
		"{{ site }}",
		vec![
			(
				Ctx::try_from_yaml(r#"
site:
  parameters:
    sharable: false
  templates:
    prototypeId: module:pages/prototype
    availability:
      templates:
        home:
          id: module:pages/home
        content:
          id: module:pages/content
  mappings:
    website:
      handlePrefix: /site
      repository: website
  theme:
    name: site-theme
  domains:
    site:
      name: site.com
        "#).unwrap(),
				Ok(r#"{"domains": {"site": {"name": "site.com"}}, "mappings": {"website": {"handlePrefix": "/site", "repository": "website"}}, "parameters": {"sharable": false}, "templates": {"availability": {"templates": {"content": {"id": "module:pages/content"}, "home": {"id": "module:pages/home"}}}, "prototypeId": "module:pages/prototype"}, "theme": {"name": "site-theme"}}"#)
			),
			(
				Ctx::try_from_yaml(r#"
site:
  parameters:
    - true
    - 12
    - false
    - -1
    - foo
        "#).unwrap(),
				Ok("{\"parameters\": [true, 12, false, -1, \"foo\"]}")
			),
		]
	),
	(
		ctx_get_yaml_03,
		"{{ ctx }}",
		vec![
			(
				Ctx::try_from_yaml(r#"- yes"#).unwrap(),
				Ok("[\"yes\"]")
			),
			(
				Ctx::try_from_yaml(r#"
- foo:
  - true
        "#).unwrap(),
				Ok("[{\"foo\": [true]}]")
			),
			(
				Ctx::try_from_yaml(r#"
- name: obj
  b: true
  c: false
        "#).unwrap(),
				Ok("[{\"b\": true, \"c\": false, \"name\": \"obj\"}]")
			),
			(
				Ctx::try_from_yaml(r#"
ctx:
  properties:
    - name: title
      label: Title
      i18n: true
      required: true
    - name: tags
      i18n: true
        "#).unwrap(),
				Ok("{\"properties\": [{\"i18n\": true, \"label\": \"Title\", \"name\": \"title\", \"required\": true}, {\"i18n\": true, \"name\": \"tags\"}]}")
			),
		]
	),
}

macro_tests! {
	interpreter_test,
	(
		ctx_get_01,
		"{{ name }}",
		vec![
			(
				Ctx::try_from(r#"{"name": "Marvin"}"#).unwrap(),
				Ok("Marvin")
			),
			(
				Ctx::try_from(r#"{"name": true}"#).unwrap(),
				Ok("true")
			),
			(
				Ctx::try_from(r#"{"name": 1}"#).unwrap(),
				Ok("1")
			),
			(
				Ctx::try_from(r#"{"name": null}"#).unwrap(),
				Ok("null")
			),
			(
				Ctx::try_from(r#"{}"#).unwrap(),
				Err(vec!["name", "not", "found"])
			),
			(
				Ctx::try_from(r#"{"name": {"first": "Marvin"}}"#).unwrap(),
				Ok(r#"{"first": "Marvin"}"#)
			),
			(
				Ctx::try_from(r#"{"name": ["Marvin"]}"#).unwrap(),
				Ok("[\"Marvin\"]")
			),
			(
				Ctx::try_from(r#"{"name": {"first": "Marvin", "last": ["foo", "bar"]}}"#).unwrap(),
				Ok(r#"{"first": "Marvin", "last": ["foo", "bar"]}"#)
			),
			(
				Ctx::try_from(r#"{"name": [null, true, false, 1, 1.2, "Marvin", {}, []]}"#).unwrap(),
				Ok("[null, true, false, 1, 1.2, \"Marvin\", {}, []]")
			)
		]
	),
	(
		ctx_get_02,
		"{{ usr.name }}",
		vec![
			(
				Ctx::try_from(r#"{"usr": {"name": "Marvin"}}"#).unwrap(),
				Ok("Marvin")
			),
			(
				Ctx::try_from(r#"{"usr": {"n": "Marvin"}}"#).unwrap(),
				Err(vec!["name", "not", "found"])
			),
			(
				Ctx::try_from(r#"{}"#).unwrap(),
				Err(vec!["usr", "not", "found"])
			),
			(
				Ctx::try_from(r#"{"usr": "Marvin"}"#).unwrap(),
				Err(vec!["access", "value", "name"])
			)
		]
	),
	(
		ctx_get_03,
		r#"{{ usr["name"] }}"#,
		vec![
			(
				Ctx::try_from(r#"{"usr": {"name": "Marvin"}}"#).unwrap(),
				Ok("Marvin")
			),
			(
				Ctx::try_from(r#"{"usr": {"n": "Marvin"}}"#).unwrap(),
				Err(vec!["name", "not", "found"])
			),
			(
				Ctx::try_from(r#"{}"#).unwrap(),
				Err(vec!["usr", "not", "found"])
			)
		]
	),
	(
		ctx_get_04,
		r#"{{ usr.info.name }}"#,
		vec![
			(
				Ctx::try_from(r#"{"usr": {"info": {"name": "Marvin"}}}"#).unwrap(),
				Ok("Marvin")
			),
			(
				Ctx::try_from(r#"{"usr": {"info": {"n": "Marvin"}}}"#).unwrap(),
				Err(vec!["name", "not", "found"])
			),
			(
				Ctx::try_from(r#"{"usr": {"inf": "Marvin"}}"#).unwrap(),
				Err(vec!["info", "not", "found"])
			),
			(
				Ctx::try_from(r#"{}"#).unwrap(),
				Err(vec!["usr", "not", "found"])
			)
		]
	),
	(
		ctx_get_arr_01,
		"{{ arr[0] }}",
		vec![
			(
				Ctx::try_from(r#"{"arr": ["Marvin"]}"#).unwrap(),
				Ok("Marvin")
			),
			(
				Ctx::try_from(r#"{"arr": ["Marvin", "C3PO", "R2D2"]}"#).unwrap(),
				Ok("Marvin")
			),
			(
				Ctx::try_from(r#"{"array": ["Marvin", "C3PO", "R2D2"]}"#).unwrap(),
				Err(vec!["arr", "not", "found"])
			),
			(
				Ctx::try_from(r#"{}"#).unwrap(),
				Err(vec!["arr", "not", "found"])
			)
		]
	),
	(
		ctx_get_arr_02,
		"{{ arr[1] }}",
		vec![
			(
				Ctx::try_from(r#"{"arr": ["Marvin"]}"#).unwrap(),
				Err(vec!["array", "bounds"])
			),
			(
				Ctx::try_from(r#"{"arr": ["Marvin", "C3PO", "R2D2"]}"#).unwrap(),
				Ok("C3PO")
			),
			(
				Ctx::try_from(r#"{"array": ["Marvin", "C3PO", "R2D2"]}"#).unwrap(),
				Err(vec!["arr", "not", "found"])
			),
			(
				Ctx::try_from(r#"{}"#).unwrap(),
				Err(vec!["arr", "not", "found"])
			)
		]
	),
	(
		ctx_get_arr_03,
		"{{ arr[0][1] }}",
		vec![
			(
				Ctx::try_from(r#"{"arr": [["Marvin", "C3PO", "R2D2"]]}"#).unwrap(),
				Ok("C3PO")
			),
			(
				Ctx::try_from(r#"{"arr": [[]]}"#).unwrap(),
				Err(vec!["array", "bounds"])
			),
			(
				Ctx::try_from(r#"{"arr": []}"#).unwrap(),
				Err(vec!["array", "bounds"])
			)
		]
	),
	(
		ctx_get_multiple_01,
		"{{ data.arr[1] }}",
		vec![
			(
				Ctx::try_from(r#"{"data": {"arr": ["Marvin", "C3PO", "R2D2"]}}"#).unwrap(),
				Ok("C3PO")
			),
			(
				Ctx::try_from(r#"{"data": {"arr": ["Marvin"]}}"#).unwrap(),
				Err(vec!["array", "bounds"])
			),
			(
				Ctx::try_from(r#"{"data": {"arr": []}}"#).unwrap(),
				Err(vec!["array", "bounds"])
			),
			(
				Ctx::try_from(r#"{}"#).unwrap(),
				Err(vec!["data", "not", "found"])
			),
			(
				Ctx::try_from(r#"{"data": {}}"#).unwrap(),
				Err(vec!["arr", "not", "found"])
			),
			(
				Ctx::try_from(r#"{"data": {"arr": null}}"#).unwrap(),
				Err(vec!["access", "value"])
			),
			(
				Ctx::try_from(r#"{"data": {"arr": {}}}"#).unwrap(),
				Err(vec!["index", "object"])
			),
			(
				Ctx::try_from(r#"{"data": []}"#).unwrap(),
				Err(vec!["key", "array"])
			)
		]
	),
	(
		ctx_get_multiple_02,
		"{{ data[0].foo }}",
		vec![
			(
				Ctx::try_from(r#"{"data": [{"foo": "bar"}]}"#).unwrap(),
				Ok("bar")
			),
			(
				Ctx::try_from(r#"{"data": []}"#).unwrap(),
				Err(vec!["array", "bounds"])
			),
			(
				Ctx::try_from(r#"{}"#).unwrap(),
				Err(vec!["data", "not", "found"])
			),
			(
				Ctx::try_from(r#"{"data": {}}"#).unwrap(),
				Err(vec!["index", "object"])
			),
			(
				Ctx::try_from(r#"{"data": [{}]}"#).unwrap(),
				Err(vec!["foo", "not", "found"])
			),
			(
				Ctx::try_from(r#"{"data": null}"#).unwrap(),
				Err(vec!["access", "value"])
			),
			(
				Ctx::try_from(r#"{"data": [[]]}"#).unwrap(),
				Err(vec!["key", "array"])
			)
		]
	),
	(
		ctx_get_multiple_03,
		"{{ data[0].foo[0].bar }}",
		vec![
			(
				Ctx::try_from(r#"{"data": [{"foo": [{"bar": "baz"}]}]}"#).unwrap(),
				Ok("baz")
			),
		]
	),
	(
		ctx_get_multiple_04,
		r#"{{ x = 0 }}{{if true}}{{ x = 1 }}{{fi}}{{ x }}"#,
		vec![
			(Ctx::new(), Ok("1"))
		]
	),
	(
		ctx_array,
		"{{ ctx?len() }}",
		vec![
			(
				Ctx::try_from(r#"{"ctx": ["Marvin", "C3PO", "R2D2"]}"#).unwrap(),
				Ok("3")
			),
			(
				Ctx::try_from(r#"[]"#).unwrap(),
				Ok("0")
			),
			(
				Ctx::try_from(r#"["Marvin", "C3PO", "R2D2"]"#).unwrap(),
				Ok("3")
			),
		]
	)
}
