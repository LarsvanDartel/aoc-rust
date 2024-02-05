pub type Result<T> = ::std::result::Result<T, AoCError>;

#[derive(Debug)]
pub enum AoCError {
    Io(::std::io::Error),
    Parse(::nom::error::Error<String>),

    NoSolution,
    Message(String),
    Unknown(String),
}

impl From<::std::io::Error> for AoCError {
    fn from(e: ::std::io::Error) -> Self {
        AoCError::Io(e)
    }
}

impl From<nom::error::Error<&str>> for AoCError {
    fn from(e: nom::error::Error<&str>) -> Self {
        AoCError::Parse(nom::error::Error::new(e.input.to_string(), e.code))
    }
}

impl From<String> for AoCError {
    fn from(e: String) -> Self {
        AoCError::Message(e)
    }
}

impl From<&str> for AoCError {
    fn from(e: &str) -> Self {
        AoCError::Message(e.to_string())
    }
}
