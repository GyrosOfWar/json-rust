![](http://terhix.com/doc/json-rust-logo-small.png)

# JSON in Rust

Parse and serialize JSON with ease.

**[Complete Documentation](http://terhix.com/doc/json/) - [Cargo](https://crates.io/crates/json) - [Repository](https://github.com/maciejhirsz/json-rust)**

# Why?

JSON is a very loose format where anything goes - arrays can hold mixed
types, object keys can change types between API calls or not include
some keys under some conditions. Mapping that to idiomatic Rust structs
introduces friction.

This crate intends to avoid that friction by using extensive static dispatch
and hiding type information behind enums, while still giving you all the safety
guarantees of safe Rust code.

```rust
let data = json::parse(r#"

{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}

"#).unwrap();

assert!(data["code"].is(200));
assert!(data["success"].is(true));
assert!(data["payload"]["features"].is_array());
assert!(data["payload"]["features"][0].is("awesome"));
assert!(data["payload"]["features"].contains("easyAPI"));

// Error resilient: non-existent values default to null
assert!(data["this"]["does"]["not"]["exist"].is_null());
```

## Easily create JSON data without defining structs

```rust
#[macro_use]
extern crate json;

fn main() {
    let data = object!{
        "a" => "bar",
        "b" => array![1, false, "foo"]
    };

    assert_eq!(json::stringify(data), r#"{"a":"bar","b":[1,false,"foo"]}"#);
}
```

## Installation

Just add it to your `Cargo.toml` file:

```toml
[dependencies]
json = "*"
```

Then import it in your `main.rs` / `lib.rs` file:

```rust
#[macro_use]
extern crate json;
```
