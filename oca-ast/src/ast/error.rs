use thiserror::Error;

#[derive(Error, Debug)]

pub enum AttributeError {
    #[error("Attribute type {0} doesn't exist")]
    UnknownAttributeType(String),
    #[error("Unexpected JSON value: {0}")]
    UnexpectedJsonValue(String),
}
