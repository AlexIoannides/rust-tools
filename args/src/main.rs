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
