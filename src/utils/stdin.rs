use num::Float;
use num::Integer;
use std::{io, str::FromStr};

pub fn read_line() -> String {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s);
    s
}

pub fn read_line_and_parse<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    Ok(T::from_str(read_line().trim())?)
}
