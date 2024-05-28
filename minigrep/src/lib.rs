use std::error::Error;
use std::fs;

pub fn run(config: GrepArgs) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

pub struct GrepArgs {
    pub query: String,
    pub file_path: String,
}

impl GrepArgs {
    pub fn build(argv: &[String]) -> Result<GrepArgs, &'static str> {
        if argv.len() < 3 {
            return Err("not enough arguments");
        }

        let query = argv[1].clone();
        let file_path = argv[2].clone();

        Ok(GrepArgs { query, file_path })
    }
}
