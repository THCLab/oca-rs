use thiserror::Error;

use super::RefValueParsingError;

#[derive(Error, Debug)]

pub enum AttributeError {
    #[error("Attribute type {0} doesn't exist")]
    UnknownAttributeType(String),
    #[error("Error while converting {0} to attribute type")]
    ConvertingFailure(String),
    #[error(transparent)]
    ReferenceError(#[from] RefValueParsingError),
}
