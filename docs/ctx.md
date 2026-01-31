## Examples

### No context:
Only the default context from stdlib will be added.
```rust
use osmia::Osmia;

let mut osmia = Osmia::default();
let output = osmia.run_code("{{ _OSMIA_VERSION }}").unwrap();
assert_eq!(output, env!("CARGO_PKG_VERSION").to_string());
```

### Json context as an object:
```rust
use osmia::Osmia;

let mut osmia = Osmia::try_from_json(r#"
{
    "user": {
        "name": "Marvin",
        "details": {
            "alias": "c3po"
        }
    }
}
"#).unwrap();
let output = osmia.run_code("Hello {{ user.name }}, aka {{ user.details.alias }}!").unwrap();
assert_eq!(output, "Hello Marvin, aka c3po!".to_string());
let output = osmia.run_code("{{ user }}").unwrap();
assert_eq!(output, "{\"details\": {\"alias\": \"c3po\"}, \"name\": \"Marvin\"}".to_string());
```

### Json context as an array:
```rust
use osmia::Osmia;

let mut osmia = Osmia::try_from_json(r#"
[
    true,
    [
        false
    ]
]
"#).unwrap();
let output = osmia.run_code("First {{ ctx[0] }}, second: {{ ctx[1][0] }}").unwrap();
assert_eq!(output, "First true, second: false".to_string());
let output = osmia.run_code("{{ ctx }}").unwrap();
assert_eq!(output, "[true, [false]]".to_string());
```

### YAML context as an object:
```rust
use osmia::Osmia;

let mut osmia = Osmia::try_from_yaml(r#"
user:
    name: Marvin
    details:
        alias: c3po
"#).unwrap();
let output = osmia.run_code("Hello {{ user.name }}, aka {{user.details.alias}}!").unwrap();
assert_eq!(output, "Hello Marvin, aka c3po!".to_string());
let output = osmia.run_code("{{ user }}").unwrap();
assert_eq!(output, "{\"details\": {\"alias\": \"c3po\"}, \"name\": \"Marvin\"}".to_string());
```

### YAML context as an array:
```rust
use osmia::Osmia;

let mut osmia = Osmia::try_from_yaml(r#"
- true
- array:
  - false
"#).unwrap();
let output = osmia.run_code("First {{ ctx[0] }}, second: {{ ctx[1].array[0] }}").unwrap();
assert_eq!(output, "First true, second: false".to_string());
let output = osmia.run_code("{{ ctx }}").unwrap();
assert_eq!(output, "[true, {\"array\": [false]}]".to_string());
```
