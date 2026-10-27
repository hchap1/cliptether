pub type Res<T> = Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    NoSuitableConfigLocation,
    InvalidConfig,
    InvalidKey
}
