# justcode-derive

Derive macros for the justcode binary encoder/decoder library.

## Overview

This crate provides procedural macros to automatically implement the `Encode` and `Decode` traits from `justcode-core` for your custom types.

## Installation

This crate is typically used as a dependency of `justcode-core` when the `derive` feature is enabled:

```toml
[dependencies]
justcode-core = { version = "0.3.0", features = ["derive"] }
```

You generally don't need to add `justcode-derive` directly to your `Cargo.toml` unless you're implementing custom derive macros.

## Usage

Use the `#[derive(Encode, Decode)]` attribute on your structs and enums:

```rust
use justcode_core::{Encode, Decode};

#[derive(Encode, Decode)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Encode, Decode)]
enum Shape {
    Circle { center: Point, radius: f32 },
    Rectangle { top_left: Point, bottom_right: Point },
}
```

### Supported Types

The derive macros support:

- **Structs**: Named fields, tuple structs, and unit structs
- **Enums**: Variants with named fields, tuple variants, and unit variants
- **Generics**: Generic structs and enums with trait bounds

### Limitations

- Unions are not supported
- Complex trait bounds may require manual implementation

## License

MIT






