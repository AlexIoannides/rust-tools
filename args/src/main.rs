use std::{env, fs, io, process};

use count::count_lines;

fn main() -> Result<(), &'static str> {
    let Some(path) = env::args().nth(1) else {
        return Err("USAGE: count <FILE>");
    };

    let file = fs::File::open(&path).unwrap_or_else(
        |e| {
            eprint!("{e}");
            process::exit(1);
        }
    );
    let lines = count_lines(io::BufReader::new(file));
    match lines {
        Ok(n) => println!("{n} lines"),
        Err(msg)=> println!("ERROR: {msg}"),
    }
    Ok(())
}
