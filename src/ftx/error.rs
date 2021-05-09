pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Api(crate::rest::Error),
    Ws(crate::ws::Error),
}

impl From<crate::rest::Error> for Error {
    fn from(err: crate::rest::Error) -> Error {
        Error::Api(err)
    }
}

impl From<crate::ws::Error> for Error {
    fn from(err: crate::ws::Error) -> Error {
        Error::Ws(err)
    }
}
