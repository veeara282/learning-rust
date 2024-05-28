use std::env;
use std::fs;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let args = GrepArgs::build(&argv).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", args.query);
    println!("In file {}", args.file_path);

    let contents = fs::read_to_string(args.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
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
