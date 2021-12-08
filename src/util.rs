use regex::Regex;
use std::str::FromStr;

pub type BResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn read_file_maybe_test(path: &str, test_path: &str) -> BResult<String> {
    if std::env::args().any(|a| a == "--test") {
        read_file(test_path)
    } else {
        read_file(path)
    }
}

pub fn read_file(path: &str) -> BResult<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn parse_lines<T: FromStr>(text: &str) -> BResult<Vec<T>>
where
    T::Err: std::error::Error + 'static,
{
    Ok(text
        .lines()
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<T>, _>>()?)
}

pub fn split_parse<T: FromStr>(text: &str, splitter: Regex) -> BResult<Vec<T>>
where
    T::Err: std::error::Error + 'static,
{
    Ok(splitter
        .split(text.trim())
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<T>, _>>()?)
}

#[derive(Debug)]
pub struct Boom {
    pub value: String,
}

impl Boom {
    pub fn from_display<T: std::fmt::Display>(disp: T) -> Boom {
        Boom::from_display_ref(&disp)
    }

    pub fn from_display_ref<T: std::fmt::Display>(disp: &T) -> Boom {
        Boom {
            value: disp.to_string(),
        }
    }
}

// Implement `Display` for `MinMax`.
impl std::fmt::Display for Boom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::error::Error for Boom {}

pub fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}
