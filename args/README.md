# Example Crate: args

Build the example:

```text
cargo build --release
```

Run the example:

```text
cat README.md | ./target/release/count
```

## Notes

While working through this section of book the error-handling control flow was evolved in several stages.

### First Principles Error Handling

All errors needs to be handled explicitly using either `match` statements or `let .. else` control flow:

```rust
use std::{env, fs, io, process};

use count::count_lines;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("USAGE: count <FILE>");
        process::exit(1);
    };

    let Ok(file) = fs::File::open(&path) else {
        eprintln!("Cannot open file.");
        process::exit(1);
    };

    let lines = count_lines(io::BufReader::new(file));
    match lines {
        Ok(n) => println!("{n} lines"),
        Err(msg) => println!("ERROR: {msg}"),
    }
}
```

### Let `main` Return `Result`

A `main` function that returns a `Result` will automatically end with an exit code of 1 or 0 depending on the output type. It shortcuts the need to do anything other than return an error.

```rust
use std::{env, fs, io};

use count::count_lines;

fn main() -> Result<(), &'static str> {
    let Some(path) = env::args().nth(1) else {
        return Err("USAGE: count <FILE>");
    };

    let Ok(file) = fs::File::open(&path) else {
        return Err("Cannot open file");
    };

    let lines = count_lines(io::BufReader::new(file));
    match lines {
        Ok(n) => println!("{n} lines"),
        Err(_msg) => return Err("ERROR: cannot read lines"),
    }
    Ok(())
}
```

Notice that because we've set the return type for errors to be `&'static str` we have to change the error path in the match statement.

### Add `?` Operator and Return `std::io::Result`

The `?` operator works within functions that return a `Result` or `Option`, and can be used with both `Option` and `Result` types. It will return the happy path if it's available, otherwise it will break control-flow to exit a function with an error. Consequently, the error type matters - e.g., in the example below it has to be of type `std::io::Error`. For type `Result` the `map_err` method can be used to marshall the error type, whereas for type `Option` there is no error (only `None`) and the error has to be marshalled explicitly using `ok_or` as demonstrated below.

```rust
use std::{env, fs, io};

use count::count_lines;

fn main() -> io::Result<()> {
    let path = env::args()
        .nth(1)
        .ok_or(io::Error::other("USAGE: count <FILE>"))?;

    let file = fs::File::open(&path)?;

    let n_lines = count_lines(io::BufReader::new(file))?;
    println!("{n_lines} lines");
    Ok(())
}
```

### Add `anyhow::Context`

The previous variants of this program demonstrate that error handling can be 'fiddly'. The anyhow crate bridges the gap from low-level Rust to something approaching the simplicity of Python's universal `Exception` type.

```rust
use std::{env, fs, io};

use anyhow::{Context, Result};

use count::count_lines;

fn main() -> Result<()> {
    let path = env::args().nth(1).context("USAGE: count <FILE>")?;
    let file = fs::File::open(&path)?;
    let n_lines = count_lines(io::BufReader::new(file))?;
    println!("{n_lines} lines");
    Ok(())
}
```
