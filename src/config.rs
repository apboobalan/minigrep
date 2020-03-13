use crate::error::LessArgumentsErr;
use std::{cmp, env};
#[derive(Debug)]
pub struct Config<'a> {
    pub file_name: &'a str,
    pub query: &'a str,
    pub case_insensitive: bool,
}

impl<'a> Config<'_> {
    pub fn from(args: &'a Vec<String>) -> Result<Config<'a>, LessArgumentsErr> {
        if args.len() < 3 {
            return Err(LessArgumentsErr);
        }
        Ok(Config {
            file_name: &args[1],
            query: &args[2],
            case_insensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

impl cmp::PartialEq for Config<'_> {
    fn eq(&self, other: &Config<'_>) -> bool {
        self.file_name.eq(other.file_name) && self.query.eq(other.query)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_valid_arguments() {
        let args = vec![
            String::from("target/debug/minigrep"),
            String::from("rust.txt"),
            String::from("Rust"),
        ];
        assert_eq!(
            Config {
                file_name: "rust.txt",
                query: "Rust",
                case_insensitive: false,
            },
            Config::from(&args).unwrap()
        );
    }
    #[test]
    fn parse_invalid_number_of_arguments() {
        let args = vec![
            String::from("target/debug/minigrep"),
            String::from("rust.txt"),
        ];

        if let Err(error) = Config::from(&args) {
            assert_eq!(error.to_string(), "Less Arguments.")
        };
    }
}
