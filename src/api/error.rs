#[derive(Debug)]
pub enum Error
{
    Message(String),
    Reqwest(reqwest::Error)
}

impl From<reqwest::Error> for Error {
    fn from(e : reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
