# Example Crate: count

A minimal Rust library and binary demonstrating a simple CLI tool that parses and processes input from stdin.

Build the example:

```text
cargo build --release
```

Run the example:

```text
cat README.md | ./target/release/count
```

Run tests:

```bash
cargo test
```

Generate and view documentation locally:

```text
cargo doc --open
```

The library source is in `src/lib.rs` and the binary entrypoint is `src/main.rs`.

## Notes

This implementation:

```rust
pub fn count_lines<T: BufRead>(input: T) -> Result<usize> {
    let mut count = 0;
    for line in input.lines(){
        line?;
        count +=1;
    }
    Ok(count)
}
```

This implementation depends on the `BufRead` trait, which provides the `lines` method. The `BufRead` trait extends the `Read` trait, and `lines` is available for `Sized` implementers.  It return an iterator of `std::io::Result<String>` that is an alias for `Result<T, std::io:Error>`.
