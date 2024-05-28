use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let config = GrepArgs::build(&argv).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: GrepArgs) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

struct GrepArgs {
    query: String,
    file_path: String,
}

impl GrepArgs {
    fn build(argv: &[String]) -> Result<GrepArgs, &'static str> {
        if argv.len() < 3 {
            return Err("not enough arguments");
        }

        let query = argv[1].clone();
        let file_path = argv[2].clone();

        Ok(GrepArgs { query, file_path })
    }
}
