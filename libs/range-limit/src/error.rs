use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    TooLarge { require: usize, get: usize },
    TooSmall { require: usize, get: usize },
    FixSize { require: usize, get: usize },
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TooLarge { require, get } => {
                write!(
                    f,
                    "Out of Length Limit: require < {} but get {}",
                    require, get
                )
            }
            Error::TooSmall { require, get } => {
                write!(
                    f,
                    "Out of Length Limit: require > {} but get {}",
                    require, get
                )
            }
            Error::FixSize { require, get } => {
                write!(f, "require Size == {require} but get {get}")
            }
        }
    }
}
