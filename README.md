# kvstruct

A minimalistic, zero-dependency Rust library for generating JSON-like representations of your structs.

Instead of relying on heavy procedural macros or external dependencies like `serde`, `kvstruct` provides a simple declarative macro (`json!`) that wraps your struct definition and automatically implements a `to_json()` method for it.

## Features

- **Zero Dependencies**: Pure Rust, no external crates required. Fast compile times.
- **Declarative Macro**: Uses a simple `macro_rules!` to generate serialization code.
- **Easy to Use**: Just wrap your struct definition in the `json!` macro.
- **Preserves Struct Attributes**: Fully supports visibility modifiers (e.g., `pub`), attributes (e.g., `#[derive(...)]`), and standard types.

## Installation

cargo add kvstruct@1.0.0

## Usage

Use the `json!` macro to define your struct, which will automatically implement the `Jsonfy` trait.

```rust
use kvstruct::{json, Jsonfy};

json! {
    #[derive(Debug)]
    pub struct User {
        pub name: String,
        pub age: u32,
        pub is_active: bool,
    }
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
        is_active: true,
    };

    // Serialize the struct to a formatted JSON string
    let json_string = user.to_json();
    println!("{}", json_string);
}
```

### Important Note on Serialization

Behind the scenes, the `json!` macro uses the `std::fmt::Debug` (`{:?}`) representation to serialize the values of your struct fields. 
Therefore, **all field types must implement `Debug`**. While this works perfectly for strings, numbers, booleans, and many standard types to generate JSON-like output, please be aware that complex nested structures or custom `Debug` implementations might not produce strict, RFC-compliant JSON. This library is best meant for simple, fast serialization or debugging purposes.

## License

MIT OR Apache-2.0
