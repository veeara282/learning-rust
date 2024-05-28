use std::env;
use std::process;

use minigrep::GrepArgs;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let config = GrepArgs::build(&argv).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
