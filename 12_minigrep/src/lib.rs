use std::env;
use std::fs;
use std::io::Error;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    fn read_option(args: &[String]) -> Result<String, ()> {
        if args.len() >= 4 {
            Ok(args[3].clone())
        } else {
            Err(())
        }
    }

    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let option = match args.next() {
            Some(arg) => arg,
            None => String::from(""),
        };

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // let query = args[1].clone();
        // let filename = args[2].clone();
        // let option = Config::read_option(args).unwrap_or_default();

        let case_sensitive;
        if option == "-i" {
            case_sensitive = false;
        } else {
            case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Error> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.trim())
        .collect()

    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line.trim());
    //     }
    // }

    // results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.trim())
        .collect()

    // let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line.trim());
    //     }
    // }

    // results
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn error_for_less_than_three_args() {
        let args = [];
        let error = Config::new(&args).unwrap_err();

        assert_eq!(error, "not enough arguments");
    }

    #[test]
    fn ok_for_three_args() {
        let args = [
            String::from("this"),
            String::from("is"),
            String::from("Sparta!"),
        ];
        let result = Config::new(&args).unwrap();

        assert_eq!(result.query, "is");
        assert_eq!(result.filename, "Sparta!");
        assert_eq!(result.case_sensitive, true);
    }

    #[test]
    fn read_option_argument() {
        let args = [
            String::from("this"),
            String::from("is"),
            String::from("Sparta!"),
            String::from("-i"),
        ];
        let result = Config::new(&args).unwrap();

        assert_eq!(result.query, "is");
        assert_eq!(result.filename, "Sparta!");
        assert_eq!(result.case_sensitive, false);
    }

    #[test]
    fn read_option_env() {
        let args = [
            String::from("this"),
            String::from("is"),
            String::from("Sparta!"),
        ];

        std::env::set_var("CASE_INSENSITIVE", "1");

        let result = Config::new(&args).unwrap();

        assert_eq!(result.query, "is");
        assert_eq!(result.filename, "Sparta!");
        assert_eq!(result.case_sensitive, false);
    }

    #[test]
    fn read_option_argument_over_env() {
        let args = [
            String::from("this"),
            String::from("is"),
            String::from("Sparta!"),
            String::from("-i"),
        ];

        std::env::set_var("CASE_INSENSITIVE", "1");

        let result = Config::new(&args).unwrap();

        assert_eq!(result.query, "is");
        assert_eq!(result.filename, "Sparta!");
        assert_eq!(result.case_sensitive, false);
    }

    #[test]
    fn error_for_missing_file() {
        let error = run(Config {
            query: String::from("query"),
            filename: String::from("not_a_real_file.txt"),
            case_sensitive: false,
        })
        .unwrap_err();

        let kind = error.kind();

        assert_eq!(kind, ErrorKind::NotFound);
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
