#[derive(Debug, PartialEq)]
pub enum Error {
    General(String),
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

impl From<scan_fmt::parse::ScanError> for Error {
    fn from(e: scan_fmt::parse::ScanError) -> Self {
        Error::Parse(e.to_string())
    }
}
