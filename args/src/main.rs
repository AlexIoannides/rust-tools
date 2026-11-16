use std::{env, fs, io, process};

use count::count_lines;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("USAGE: count <FILE>");
        process::exit(1);
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
}
