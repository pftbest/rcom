use std::io;
use std::fmt;
use std::error;
use std::convert::From;

#[derive(Debug)]
pub struct CustomError(String);

impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> CustomError {
        CustomError(err.to_string())
    }
}

impl fmt::Display for CustomError {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> Result<(), fmt::Error> {
        write!(f, "Error: {}", self.0)
    }
}

impl error::Error for CustomError {
    fn description<'a>(&'a self) -> &'a str {
        &self.0[..]
    }
}
