pub type Res<T> = Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    NoSuitableConfigLocation,
    InvalidConfig,
    InvalidKey,
    Io,
    Clipboard,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::Io
    }
}
