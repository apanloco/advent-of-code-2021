#[derive(Debug, PartialEq)]
pub enum Error {
    Parse(String),
    Io(String),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::Parse(e.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::Parse(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}
