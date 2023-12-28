pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(::std::io::Error),
    Parse(::nom::error::Error<String>),

    Message(String),
    Unknown(String),
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<nom::error::Error<&str>> for Error {
    fn from(e: nom::error::Error<&str>) -> Self {
        Error::Parse(nom::error::Error::new(e.input.to_string(), e.code))
    }
}
