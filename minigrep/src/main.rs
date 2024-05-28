use std::env;
use std::fs;

// struct GrepArgs<'a> {
//     query: &'a str,
//     file_path: &'a str,
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// fn parse_config_struct(args: &[String]) -> GrepArgs {
//     GrepArgs { query: &args[1], file_path: &args[2] }
// }

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    return (query, file_path)
}
