use std::{env, fs, process};

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("USAGE: count <FILE>");
        process::exit(1);
    };

    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprint!("{e}");
            process::exit(1);
        },
    };
    println!("{:?}", file)
}
