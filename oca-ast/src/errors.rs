use thiserror::Error;

#[derive(Error, Debug)]

pub enum Error {
    #[error("{0}")]
    InvalidVersion(String),

    #[error("{0}")]
    InvalidOperation(String),

    #[error("{0}")]
    Unknown(String),

    #[error("")]
    MissingVersion(),

    #[error("Validation errors: {0:?}")]
    Validation(Vec<Error>),
}

struct Errors(Vec<Error>);

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, error| {
            result.and_then(|_| writeln!(f, "{}", error))
        })
    }
}