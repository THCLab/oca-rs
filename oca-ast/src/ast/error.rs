use thiserror::Error;

#[derive(Error, Debug)]

pub enum AttributeError {
    #[error("Attribute type {0} doesn't exist")]
    UnknownAttributeType(String),
    #[error("Error while converting {0} to attribute type")]
    ConvertingFailure(String),
    #[error("Invalid said: {0}")]
    SaidError(String)
}
