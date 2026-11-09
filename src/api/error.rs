pub enum Error
{
    Message(String)
}

pub type Result<T> = std::result::Result<T, Error>;
