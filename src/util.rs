use std::str::FromStr;

pub type BResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
