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
