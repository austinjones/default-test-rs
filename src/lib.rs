//! Provides a Rust trait similar to [Default](https://doc.rust-lang.org/std/default/trait.Default.html) that can be used in tests.
//!
//! Often tests need to construct mock instances of structs.  For example:
//! ```rust
//! struct User {
//!     id: usize,
//!     name: String,
//!     email: String,
//!     admin: bool
//! }
//! ```
//!
//! While it's tempting to define Default, often tests need mock values for types that
//! shouldn't apply in production code.  Sometimes, tests need values for types that don't even implement Default.
//!
//! This crate provides [DefaultTest](./trait.DefaultTest.html), a trait which can provide default instances with mock values.
//! Tests can construct instances, and use the spread operator to override values.
//! ```rust
//! # struct User {
//! #    id: usize,
//! #    name: String,
//! #    email: String,
//! #    admin: bool
//! # }
//! use default_test::DefaultTest;
//!
//! impl DefaultTest for User {
//!     fn default_test() -> Self {
//!         User {
//!             id: 0,
//!             name: "name".into(),
//!             email: "email".into(),
//!             admin: false
//!         }
//!     }
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     #[test]
//!     fn test() {
//!         let user = User {
//!             id: 99
//!             ..User::test_default()
//!         };
//!         // ...
//!     }
//! }
//! ```
//!
//! This style makes tests much more stable, and when adding a field to a struct, it reduces the amount of required edits in your unit tests.
//!
//! ## Roadmap:
//! - Derive macro which fills sensible defaults that would be useful in unit test implementations.  
//! String files would be filled with their property name, and other types may use T::default() or unique values.

/// A trait for giving a type a useful default value, in the scope of unit tests.
///
/// Sometimes, unit tests need to construct a mock value when working with structs, such as:
/// ```rust
/// struct User {
///     id: usize,
///     name: String,
///     email: String,
///     admin: bool
/// }
/// ```
///
/// DefaultTest can be used to define default mocked values for use in tests.
/// Tests can construct instances, and use the spread operator to override values.
/// ```rust
/// # struct User {
/// #    id: usize,
/// #    name: String,
/// #    email: String,
/// #    admin: bool
/// # }
/// use default_test::DefaultTest;
///
/// impl DefaultTest for User {
///     fn default_test() -> Self {
///         User {
///             id: 0,
///             name: "name".into(),
///             email: "email".into(),
///             admin: false
///         }
///     }
/// }
///
/// #[cfg(test)]
/// mod tests {
///     #[test]
///     fn test() {
///         let user = User {
///             id: 99
///             ..User::test_default()
///         };
///         // ...
///     }
/// }
/// ```
pub trait DefaultTest {
    /// Returns a "default value" for a type, containing mocked values suitable for use in tests.
    /// Default values may contain literals, unique numbers, etc, to make test assertions easier to work with.
    /// # Examples
    /// ```
    /// use default_test::DefaultTest;
    /// let x: String = DefaultTest::default_test();
    /// ```
    /// Make your own:
    /// ```
    /// use default_test::DefaultTest;
    ///
    /// struct Foo {
    ///     bar: String   
    /// }
    ///
    /// impl DefaultTest for Foo {
    ///     fn default_test() -> Self {
    ///         Self {
    ///             bar: "bar".into()
    ///         }
    ///     }  
    /// }
    fn default_test() -> Self;
}

impl DefaultTest for bool {
    fn default_test() -> Self {
        false
    }
}

impl DefaultTest for char {
    fn default_test() -> Self {
        '-'
    }
}

impl DefaultTest for &str {
    fn default_test() -> Self {
        "string"
    }
}

impl DefaultTest for String {
    fn default_test() -> Self {
        "string".into()
    }
}

impl DefaultTest for usize {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for isize {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for u8 {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for i8 {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for u16 {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for i16 {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for u32 {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for i32 {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for u64 {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for i64 {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for u128 {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for i128 {
    fn default_test() -> Self {
        0
    }
}

impl DefaultTest for f32 {
    fn default_test() -> Self {
        0.0
    }
}

impl DefaultTest for f64 {
    fn default_test() -> Self {
        0.0
    }
}

// impl<T, E> DefaultTest for Result<T, E>
// where
//     T: DefaultTest,
// {
//     fn default_test() -> Self {
//         Ok(T::default_test())
//     }
// }

// impl<T> DefaultTest for Option<T>
// where
//     T: DefaultTest,
// {
//     fn default_test() -> Self {
//         Some(T::default_test())
//     }
// }
