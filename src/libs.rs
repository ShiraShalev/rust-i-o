use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub not_case_sensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        } else if args.len() > 3 {
            return Err("Unknown argument provided");
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
