//! The executable entry point for the hello example.
//!
//! This binary delegates its behaviour to the library's `greeting` function.

use hello::greeting;

fn main() {
    let msg = greeting();
    println!("{msg}");
}
