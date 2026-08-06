//! The executable entry point for the count example.
//!
//! Run this with `echo YOUR\nINPUT | cargo run` or `cargo run << 'EOF'`.
//! So that the stdin stream receives the EOF signal.

use std::io::{stdin, BufRead};

fn main() {
    let input = stdin().lock();
    let line_count = input.lines().count();
    println!("{line_count} lines")
}
