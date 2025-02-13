use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub not_case_sensitive: bool,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let not_case_sensitive =
            env::var("NOT_CASE_SENSITIVE").is_ok() || args.contains(&String::from("-cs"));

        Ok(Config {
            query,
            file_path,
            not_case_sensitive,
        })
    }
}

pub fn read_from_file(file_path: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    Ok(contents)
}

pub fn get_lines_with_query(
    file_content: &String,
    query: &String,
    not_case_sensitive: &bool,
) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    for line in file_content.lines() {
        if (!not_case_sensitive && line.contains(query))
            || (*not_case_sensitive && line.to_lowercase().contains(&query.to_lowercase()))
        {
            println!("{line}");
            results.push(line.to_string());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_string() {
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."
            .to_string();
        let query = String::from("three");
        assert_eq!(
            get_lines_with_query(&file_content, &query, &false),
            ["Pick three.".to_string()]
        );
    }

    #[test]
    fn find_string_not_cs() {
        let file_content = "\
Rust:
safe, fast, productive, pick.
Pick three.
Duct tape."
            .to_string();
        let query = String::from("pick");
        assert_eq!(
            get_lines_with_query(&file_content, &query, &true),
            [
                "safe, fast, productive, pick.".to_string(),
                "Pick three.".to_string(),
            ]
        );
    }

    #[test]
    fn dont_find_string() {
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."
            .to_string();
        let query = String::from("worlds");
        assert_eq!(
            get_lines_with_query(&file_content, &query, &false),
            Vec::<String>::new()
        );
    }
}

// For PR
