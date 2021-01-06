# default-test-rs
Provides a Rust trait similar to [Default](https://doc.rust-lang.org/std/default/trait.Default.html) that can be used in tests.

Often tests need to construct mock instances of structs.  For example:
```rust
struct User {
    id: usize,
    name: String,
    email: String,
    admin: bool
}
```

While it's tempting to define Default, often tests need mock values for types that
shouldn't apply in production code.
Sometimes, tests need values for types that don't even implement Default.

This crate provides DefaultTest, a trait which can provide default instances with mock values.
```rust
impl DefaultTest for User {
    fn default_test() -> Self {
        User {
            id: 0,
            name: "name".into(),
            email: "email".into(),
            admin: false
        }
    }
}
```

Unit tests can then use the spread operator to construct values:
```rust
mod tests {
    #[test]
    fn test() {
        let user = User {
            id: 99
            ..User::test_default()
        };
        // ...
    }
}
```

## Roadmap:
- Derive macro which fills sensible defaults that would be useful in unit test implementations.  String files would be filled with their property name, and other types may use T::default() or unique values.