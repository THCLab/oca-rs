use thiserror::Error;

#[derive(Error, Debug)]

pub enum AttributeError {
    #[error("{0}")]
    General(String),
}
