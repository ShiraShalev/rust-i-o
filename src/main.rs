mod libs;
use libs::Config;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprint!("Application error: {e}");
        process::exit(1)
    }
}
fn run(config: Config) -> Result<Box<[String]>, Box<dyn std::error::Error>> {
    let file = fs::File::open(config.file_path)?;
    let reader = io::BufReader::new(file);
    let result = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| {
            let match_found = if config.not_case_sensitive {
                line.to_lowercase().contains(&config.query.to_lowercase())
            } else {
                line.contains(&config.query)
            };
            if match_found {
                println!("{}", line);
            }
            match_found
        })
        .collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_string() {
        let query = String::from("how");
        let result: Box<[String]> = Box::new([]);
        assert_eq!(
            run(Config {
                query,
                file_path: "poem.txt".to_string(),
                not_case_sensitive: false
            })
            .unwrap(),
            result
        );
    }

    #[test]
    fn find_string_not_cs() {
        let query = String::from("how");
        let result: Box<[String]> = Box::new([
            "How dreary to be somebody!".to_string(),
            "How public, like a frog".to_string(),
        ]);
        assert_eq!(
            run(Config {
                query,
                file_path: "poem.txt".to_string(),
                not_case_sensitive: true
            })
            .unwrap(),
            result
        );
    }

    #[test]
    fn dont_find_string() {
        let query = String::from("how");
        let result: Box<[String]> = Box::new([]);
        assert_eq!(
            run(Config {
                query,
                file_path: "poem.txt".to_string(),
                not_case_sensitive: false
            })
            .unwrap(),
            result
        );
    }
}
