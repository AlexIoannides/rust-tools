//! The executable entry point for the hello example.
//!
//! This binary delegates its behaviour to the library's `print` function.

use hello::print;

fn main() {
    print();
}
