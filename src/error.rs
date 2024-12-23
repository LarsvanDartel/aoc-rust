pub type Result<T> = ::std::result::Result<T, AoCError>;

#[derive(Debug)]
pub enum AoCError {
    Io(::std::io::Error),
    Parse(::winnow::error::ErrMode<::winnow::error::ContextError>),
    ParseInt(::std::num::ParseIntError),

    NoSolution,
    Message(String),
    Unknown(String),
}

impl From<::std::io::Error> for AoCError {
    fn from(e: ::std::io::Error) -> Self {
        AoCError::Io(e)
    }
}

impl From<::winnow::error::ErrMode<::winnow::error::ContextError>> for AoCError {
    fn from(e: ::winnow::error::ErrMode<::winnow::error::ContextError>) -> Self {
        AoCError::Parse(e)
    }
}

impl From<::std::num::ParseIntError> for AoCError {
    fn from(e: ::std::num::ParseIntError) -> Self {
        AoCError::ParseInt(e)
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
