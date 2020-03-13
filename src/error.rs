use std::{error, fmt, process};
#[derive(Debug, Clone)]
pub struct LessArgumentsErr;

impl fmt::Display for LessArgumentsErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Less Arguments.")
    }
}

impl error::Error for LessArgumentsErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub trait ExitOnError<T> {
    fn else_exit_on_error(self) -> T;
}

impl<T, E: error::Error> ExitOnError<T> for Result<T, E> {
    fn else_exit_on_error(self) -> T {
        self.unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1)
        })
    }
}
