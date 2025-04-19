## Expression
A expression is the smallest building block of the language.

### Types:
```rust
use osmia::Osmia;

let mut osmia = Osmia::default();
// Primitives
assert_eq!(osmia.run_code("Int: {{ 1 }}").unwrap(), "Int: 1".to_string());
assert_eq!(osmia.run_code("Float: {{ 1.1 }}").unwrap(), "Float: 1.1".to_string());
assert_eq!(osmia.run_code("Bool: {{ true }} {{ false }}").unwrap(), "Bool: true false".to_string());
assert_eq!(osmia.run_code("Null: {{ null }}").unwrap(), "Null: null".to_string());
assert_eq!(osmia.run_code(r#"Str: {{ "hello" }}"#).unwrap(), "Str: hello".to_string());

// Collections
assert_eq!(osmia.run_code("Array: {{ [1, 2, 3] }}").unwrap(), "Array: [1, 2, 3]".to_string());
assert_eq!(osmia.run_code(r#"Object: {{ { "a": 1, "b": 2 } }}"#).unwrap(), r#"Object: {"a": 1, "b": 2}"#.to_string());

// Variable
assert_eq!(osmia.run_code("{{ a = 2 }}{{ a }}").unwrap(), "2".to_string());
assert_eq!(osmia.run_code("{{ math.PI }}").unwrap(), "3.141592653589793".to_string());

// Callables
assert_eq!(osmia.run_code("function call {{ math.max(1, 2) }}").unwrap(), "function call 2".to_string());
assert_eq!(osmia.run_code(r#"method call {{ true?then("Yes", "No") }}"#).unwrap(), "method call Yes".to_string());
assert_eq!(osmia.run_code("lambda {{ fn (x, y) => x + y }}").unwrap(), "lambda fn (x, y) => x + y".to_string());

// Grouping
assert_eq!(
	osmia.run_code(r#"{{ (1 * 4) * 2 }}"#).unwrap(),
	osmia.run_code(r#"{{ 1 * (4 * 2) }}"#).unwrap()
);
```

### Operations:
- [Addition](#method.add)
- [Subtraction](#method.sub)
- [Multiplication](#method.mul)
- [Division](#method.div)
- [Module / Remainder](#method.rem)
- [Comparison](#method.partial_cmp)
- Bitwise operations: [and](#method.bitand), [or](#method.bitor), [xor](#method.bitxor), [shl](#method.shl), [shr](#method.shr), [not](#method.not), [neg](#method.neg)
