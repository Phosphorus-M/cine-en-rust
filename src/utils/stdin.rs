use std::fmt;
use std::io::{self, stdin};
use std::str::FromStr;

#[derive(Debug)]
pub enum ReadLineNumError {
    IO(io::Error),
    Parse(String),
}

impl fmt::Display for ReadLineNumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ReadLineNumError::IO(err) => write!(f, "{}", err.to_string()),
            ReadLineNumError::Parse(value) => {
                write!(f, "An error ocurred parsing: {}", value)
            }
        }
    }
}

pub fn read_line_and_parse<T: FromStr>() -> Result<T, ReadLineNumError> {
    let ref mut buf: String = String::new();

    stdin()
        .read_line(buf)
        .map_err(|err| ReadLineNumError::IO(err))?;

    buf.parse::<T>()
        .map_err(|_| ReadLineNumError::Parse(buf.to_owned()))
}
