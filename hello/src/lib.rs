//! A tiny Rust crate that exposes a simple greeting function.
//!
//! This project demonstrates the standard layout for a minimal library and
//! binary pair, where the binary reuses functionality from the library.

/// Returns a greeting message.
///
/// # Examples
///
/// ```
/// use hello::greeting;
///
/// assert_eq!(greeting(), "Hello, world!");
/// ```
pub fn greeting() -> String {
    String::from("Hello, world!")
}

#[test]
fn greeting_returns_message() {
    let expected = String::from("Hello, world!");
    let actual = greeting();
    assert!(actual == expected, "expected greeting to be \"{expected}\", got \"{actual}\"");
}
