use std::env;
use std::fs;

struct GrepArgs<'a> {
    query: &'a str,
    file_path: &'a str,
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    let args = parse_config(&argv);

    println!("Searching for {}", args.query);
    println!("In file {}", args.file_path);

    let contents = fs::read_to_string(args.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn parse_config(argv: &[String]) -> GrepArgs {
    GrepArgs { query: &argv[1], file_path: &argv[2] }
}
