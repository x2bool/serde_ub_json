# Serde UBJSON (Universal Binary JSON) ![Latest Version](https://img.shields.io/crates/v/serde_ub_json.svg)

**serde_ub_json** is [UBJSON](https://ubjson.org) serialization format implementation for Rust/Serde

# Why

Structure: this format is structurally equivalent to JSON, and has the same set of data types (**numbers, booleans, strings, arrays and objects**).

Size: null, true and false values are **75% smaller**, large numeric values are **~50% smaller**, array and object containers are **1-byte-per-value smaller**.

Parsing: boolean and numeric values are not encoded as strings - they are represented as bytes and do not require parsing, strings are byte arrays with length, which means they can always be safely deserialized as string slices without data copying.

# How to install

```shell
cargo add serde_ub_json
```

Or add as a dependency in Cargo.toml:

```toml
[dependencies]
serde_ub_json = "0.1"
```

# How to use

```rust
use serde::{Deserialize, Serialize};
use serde_ub_json::{Result, to_bytes, from_bytes};

//...

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

//...

let p = Person { name: "John Doe", age: 43 };

let bytes = to_bytes(&p)?; // serialize
let person = from_bytes::<'_, Person>(&bytes)?; // deserialize

assert_eq!(p.name, person.name);
assert_eq!(p.age, person.age);
```

# About

This projects aims to be a complete implementation of UBJSON standard.
