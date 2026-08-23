//! The executable entry point for the count example.
//!
//! Run this with `echo YOUR\nINPUT | cargo run` or `cargo run << 'EOF'`.
//! So that the stdin stream receives the EOF signal.

use std::io::stdin;

use count::count_lines;

fn main() {
    let lc = count_lines(stdin().lock());
    match lc {
        Ok(n) => println!("{n} lines"),
        Err(msg)=> println!("ERROR: {msg}"),
    }
}
