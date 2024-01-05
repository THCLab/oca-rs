use thiserror::Error;

#[derive(Error, Debug)]

pub enum Error {
    #[error("{0}")]
    UnexpectedToken(String),

    #[error("{0}")]
    Parser(String),

    #[error("{0}")]
    Unknown(String),

    #[error(transparent)]
    ExtractError(#[from] ExtractingAttributeError),
}

#[derive(Error, Debug)]

pub enum ExtractingAttributeError {
    #[error(transparent)]
    SaidError(#[from] said::error::Error),

    #[error("{0}")]
    Unexpected(String),
}
