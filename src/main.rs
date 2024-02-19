use std::{env, process};

fn main() {
    let file = env::args().skip(1).next().unwrap_or_else(|| {
        eprintln!("No file provided");
        process::exit(1);
    });

    if let Err(message) = quark::compile(file) {
        eprintln!("Error: {message}");
        process::exit(1);
    }
}
