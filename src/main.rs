mod libs;
use libs::Config;
use libs::{get_lines_with_query, read_from_file};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    match read_from_file(&config.file_path) {
        Ok(contents) => {
            get_lines_with_query(&contents, &config.query, &config.not_case_sensitive);
        }
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    }
}

// For PR
