use std::{env, process};

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("USAGE: count <FILE>");
        process::exit(1);
    };

    println!("path = {:?}", path);
}
