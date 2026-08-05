//! A tiny Rust crate that exposes a simple greeting function.
//!
//! This project demonstrates the standard layout for a minimal library and
//! binary pair, where the binary reuses functionality from the library.

/// Prints the project's greeting message to standard output.
///
/// # Examples
///
/// ```
/// use hello::print;
///
/// print();
/// ```
pub fn print() {
    println!("Hello, world");
}
