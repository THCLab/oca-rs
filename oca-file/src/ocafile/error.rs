use thiserror::Error;

#[derive(Error, Debug)]

pub enum Error {
    #[error("{0}")]
    UnexpectedToken(String),

    #[error("{0}")]
    Parser(String),

    #[error("{0}")]
    Unknown(String),
}
