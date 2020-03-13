use crate::config::Config;
use crate::error::ExitOnError;
use std::{env, fs, io};

pub fn run() -> () {
    let args = env::args().collect();
    let config = Config::from(&args).else_exit_on_error();
    grep_lines_from_file(&config);
}

fn grep_lines_from_file(config: &Config) -> () {
    let file_contents = read_file(config.file_name).else_exit_on_error();
    let grepped_lines = if config.case_insensitive {
        grep_lines(&file_contents, config.query)
    } else {
        case_insensitive_grep_lines(&file_contents, config.query)
    };
    print_grepped_lines(&grepped_lines);
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

pub fn grep_lines<'a>(file_contents: &'a str, query: &str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn case_insensitive_grep_lines<'a>(file_contents: &'a str, query: &str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

fn print_grepped_lines(lines: &Vec<&str>) -> () {
    for line in lines {
        println!("{}", line);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_grep_lines() {
        let file_contents = "Hello world!\nTrhello user!";
        let query = "hello";
        assert_eq!(vec!["Trhello user!"], grep_lines(&file_contents, &query));
    }

    #[test]
    fn test_case_insensitive_grep() {
        let file_contents = "Hello world!\nTrhello user!";
        let query = "hello";
        assert_eq!(
            vec!["Hello world!", "Trhello user!"],
            case_insensitive_grep_lines(&file_contents, &query)
        );
    }
}
