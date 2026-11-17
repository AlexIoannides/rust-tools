use std::{env, fs, io};

fn main() -> anyhow::Result<()> {
    let path = env::args()
        .nth(1)
        .ok_or(io::Error::other("USAGE: count <FILE>"))?;
    let file = fs::File::open(&path)?;
    let n_lines = count::count_lines(io::BufReader::new(file))?;
    println!("{n_lines} lines");
    Ok(())
}
