use thiserror::Error;

#[derive(thiserror::Error, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum ParseError {
    #[error("Error at line {line_number} ({raw_line}): {message}")]
    GrammarError {
        #[serde(rename = "ln")]
        line_number: usize,
        #[serde(rename = "col")]
        column_number: usize,
        #[serde(rename = "c")]
        raw_line: String,
        #[serde(rename = "e")]
        message: String,
    },
    #[error("Error parsing meta: {0}")]
    MetaError(String),

    #[error("Error parsing instruction: {0}")]
    InstructionError(#[from] InstructionError),

    #[error("{0}")]
    Custom(String),
}

#[derive(Error, Debug, serde::Serialize)]
pub enum InstructionError {
    #[error("{0}")]
    UnexpectedToken(String),

    #[error("{0}")]
    Parser(String),

    #[error("{0}")]
    Unknown(String),

    #[error(transparent)]
    ExtractError(#[from] ExtractingAttributeError),
}

#[derive(Error, Debug, serde::Serialize)]
pub enum ExtractingAttributeError {
    #[error("Said error: {0}")]
    SaidError(#[from] said::error::Error),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
