## Examples
```rust
use osmia::Osmia;

let mut osmia = Osmia::default();
let output = osmia.run_code("1 + 1 = {{ 1 + 1 }}").unwrap();
assert_eq!(output, "1 + 1 = 2".to_string());
```

### Json context:
```rust
use osmia::Osmia;

let mut osmia = Osmia::try_from(r#"{ "name": "Marvin" }"#).unwrap();
let output = osmia.run_code("Hello {{ name }}!").unwrap();
assert_eq!(output, "Hello Marvin!".to_string());
```
