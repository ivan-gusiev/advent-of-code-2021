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

#[derive(Debug)]
pub struct Boom {
    pub value: String
}

impl Boom {
    pub fn from_display<T : std::fmt::Display>(disp : T) -> Boom {
        Boom::from_display_ref(&disp)
    }
    
    pub fn from_display_ref<T : std::fmt::Display>(disp : &T) -> Boom {
        Boom { value: disp.to_string() }
    }
}

// Implement `Display` for `MinMax`.
impl std::fmt::Display for Boom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::error::Error for Boom {

}
