use std::{io, str::FromStr};
use num::Integer;
use num::Float;

pub fn read_line() -> String {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s);
    s
}

pub fn read_line_and_parse<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    Ok(T::from_str(read_line().trim())?)
}

