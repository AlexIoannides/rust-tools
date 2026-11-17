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
