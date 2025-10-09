use std::{
    env, fs,
    io::{Error, ErrorKind},
};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub result_limit: Option<u32>, // determines how many lines containing query should be returned
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Error> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::new(ErrorKind::Other, "Query string not found!"))
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::new(ErrorKind::Other, "File path not found!"))
        };

        let result_limit = match args.next() {
            Some(arg) => Some(arg.parse::<u32>().unwrap_or(0)),
            None => None
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            result_limit,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Error> {
    let contents = fs::read_to_string(&config.file_path)?;

    let search_result = search(
        config.query.as_str(),
        contents.as_str(),
        config.result_limit,
        config.ignore_case,
    );

    if !search_result.is_empty() {
        println!("Search result:\n{}", search_result);
    } else {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("'{}' not found in {}", config.query, config.file_path),
        ));
    }

    Ok(())
}

pub fn search(query: &str, contents: &str, result_limit: Option<u32>, ignore_case: bool) -> String {
    let contents_vector = contents
        .lines()
        .filter(|x| {
            if ignore_case {
                x.to_lowercase().contains(&query.to_lowercase())
            } else {
                x.contains(query)
            }
        })
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    let result = match result_limit {
        Some(val) if val > 0 && val <= contents_vector.len() as u32 => {
            &contents_vector[..(val as usize)]
        }

        _ => &contents_vector,
    };

    result.join("\n").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_success() {
        let args = [
            "program".to_string(),
            "hello".to_string(),
            "file.txt".to_string(),
            "2".to_string(),
        ].into_iter();

        assert_eq!(
            Config {
                query: "hello".to_string(),
                file_path: "file.txt".to_string(),
                result_limit: Some(2),
                ignore_case: false
            },
            Config::build(args).unwrap()
        )
    }

    #[test]
    #[should_panic(expected = "Query string not found!")]
    fn config_build_failure() {
        Config::build([].into_iter()).unwrap();
    }

    #[test]
    fn search_case_sensitive() {
        let query = "Hello";
        let contents = "\
            My Rust Program:
            Hello, John,
            Hello, Mr. Doe,
            Hello. You.
        ";

        assert_eq!(
            "Hello, John,\nHello, Mr. Doe,\nHello. You.".to_string(),
            search(query, contents, Some(0), false)
        );
        assert_eq!(
            "Hello, John,".to_string(),
            search(query, contents, Some(1), false)
        );
        assert_eq!(
            "Hello, John,\nHello, Mr. Doe,".to_string(),
            search(query, contents, Some(2), false)
        );
        assert_eq!(
            "Hello, John,\nHello, Mr. Doe,\nHello. You.".to_string(),
            search(query, contents, Some(3), false)
        );
        assert_eq!(
            "Hello, John,\nHello, Mr. Doe,\nHello. You.".to_string(),
            search(query, contents, Some(4), false)
        );
    }

    #[test]
    fn search_case_insensitive() {
        let query = "hElLo";
        let contents = "\
            My Rust Program:
            Hello, John,
            Hello, Mr. Doe,
            Hello. You.
        ";

        assert_eq!(
            "Hello, John,\nHello, Mr. Doe,\nHello. You.".to_string(),
            search(query, contents, Some(0), true)
        );
        assert_eq!(
            "Hello, John,".to_string(),
            search(query, contents, Some(1), true)
        );
        assert_eq!(
            "Hello, John,\nHello, Mr. Doe,".to_string(),
            search(query, contents, Some(2), true)
        );
        assert_eq!(
            "Hello, John,\nHello, Mr. Doe,\nHello. You.".to_string(),
            search(query, contents, Some(3), true)
        );
        assert_eq!(
            "Hello, John,\nHello, Mr. Doe,\nHello. You.".to_string(),
            search(query, contents, Some(4), true)
        );
    }
}
