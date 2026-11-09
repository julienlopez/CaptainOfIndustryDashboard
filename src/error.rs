use api::Error;

#[derive(Debug)]
pub enum Error
{
    Message(String),
    Api(api::Error)
}

pub type Result<T> = std::result::Result<T, Error>;
