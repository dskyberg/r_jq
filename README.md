
A Rust based json query utility patterned after the famous jq utility.
This app has no real value other than I wanted to be able to query
[serde_json] structures, in a similar manner to jq.

The focus of this lib is to provide query capability.  There is no plan to
support the broader scripting capability of JQ.  So, no aritimatic, and very little
data transformation support.

Most basic filter cabilities work, with the exception of the Optional, `?`, 
operator for identifiers and indexes.

# Overview
A query is a set of filters and functions that operate sequentionally on an collection of Values and produce a new collection of Values.  Each filter or function accepts a collection and return a new collection. 


# Examples

## Identity

```rust
use r_jq::jq;
use serde_json::json;

let json = r#""Hello World!""#.as_bytes();
let query_str = r#"."#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!("Hello World!")]);
```

## Object Identifier-Index: `.foo`, `.foo.bar`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"{"foo": 42, "bar": "less interesting data"}"#.as_bytes();
let query_str = r#".foo"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(42)]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"{"notfoo": true, "alsonotfoo": false}"#.as_bytes();
let query_str = r#".foo"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(null)]);
```

## Optional Object Identifier-Index: `.foo?`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"{"foo": 42, "bar": "less interesting data"}"#.as_bytes();
let query_str = r#".foo?"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(42)]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"{"notfoo": true, "alsonotfoo": false}"#.as_bytes();
let query_str = r#".foo?"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(null)]);
```

## Generic Object Index: `.[<string>]`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"{"foo": 42}"#.as_bytes();
let query_str = r#".["foo"]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(42)]);
```

## Array Index: `.[2]`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]"#.as_bytes();
let query_str = ".[0]";

let result = jq(json, query_str).expect("Failed JQ");

assert_eq!(&result, &[json!({"name": "JSON", "good": true})]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]"#.as_bytes();
let query_str = r#".[2]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(null)]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[1,2,3]"#.as_bytes();
let query_str = r#".[-2]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(2)]);
```

## Array/String Slice: `.[10:15]`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"["a","b","c","d","e"]"#.as_bytes();
let query_str = r#".[2:4]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!("c"), json!("d")]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"["a","b","c","d","e"]"#.as_bytes();
let query_str = r#".[:3]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!("a"),json!("b"), json!("c")]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#""abcdefghi""#.as_bytes();
let query_str = r#".[2:4]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!("cd")]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"["a","b","c","d","e"]"#.as_bytes();
let query_str = r#".[-2:]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!("d"),json!("e")]);
```
## Array/Object Value Iterator: `.[]`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]"#.as_bytes();
let query_str = r#".[]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!({"name":"JSON", "good":true}),json!({"name":"XML", "good":false})]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[]"#.as_bytes();
let query_str = r#".[]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &Vec::<serde_json::Value>::new());
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"{"a": 1, "b": 1}"#.as_bytes();
let query_str = r#".[]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(1),json!(1)]);
```

## `.[]?`
Coming soon

## Comma `,`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"{"foo": 42, "bar": "something else", "baz": true}"#.as_bytes();
let query_str = r#".foo, .bar"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(42), json!("something else")]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"{"user":"stedolan", "projects": ["jq", "wikiflow"]}"#.as_bytes();
let query_str = r#".user, .projects[]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!("stedolan"), json!("jq"),json!("wikiflow")]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"["a","b","c","d","e"]"#.as_bytes();
let query_str = r#".[4,2]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!("e"), json!("c")]);
```

## Pipe: `|`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]"#.as_bytes();
let query_str = r#".[] | .name"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!("JSON"), json!("XML")]);
```

## Types and Values
jq supports the same set of datatypes as JSON - numbers, strings, booleans, arrays, objects (which in JSON-speak are hashes with only string keys), and "null".

Booleans, null, strings and numbers are written the same way as in javascript. 

## Array construction: `[]`

As in JSON, `[]` is used to construct arrays, as in `[1,2,3]`. The elements of the arrays can be any jq expression, including a pipeline. All of the results produced by all of the expressions are collected into one big array. You can use it to construct an array out of a known quantity of values (as in `[.foo, .bar, .baz]`) or to "collect" all the results of a filter into an array (as in `[.items[].name]`)

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"{"user":"stedolan", "projects": ["jq", "wikiflow"]}"#.as_bytes();
let query_str = r#"[.user, .projects[]]"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(["stedolan", "jq", "wikiflow"])]);
```

## Object Construction: {}

Not yet.  Maybe never

## Math functions

Not yet.  Maybe never

## Recursive Descent `..`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[[{"a":1}]]"#.as_bytes();
let query_str = r#"..|.a?"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(1)]);
```

# Builtin operators and functions

## `length`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[[1,2], "string", {"a":2}, null]"#.as_bytes();
let query_str = r#".[] | length"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(2), json!(6), json!(1), json!(0)]);
```
## `utf8bytelength`

Not yet

## `keys`, `keys_unsorted`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"{"abc": 1, "abcd": 2, "Foo": 3}"#.as_bytes();
let query_str = r#"keys"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(["Foo", "abc", "abcd"])]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[42, 3, 35]"#.as_bytes();
let query_str = r#"keys"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!([0, 1, 2])]);
```
## `has(key)`

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[{"foo": 42}, {}]"#.as_bytes();
let query_str = r#".[] | has("foo")"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(true), json!(false)]);
```

```rust
use r_jq::jq;
use serde_json::json;

let json = r#"[[0,1], ["a","b","c"]]"#.as_bytes();
let query_str = r#".[] | has(2)"#;

let result = jq(json, query_str).expect("Failed JQ");
assert_eq!(&result, &[json!(false), json!(true)]);
```
