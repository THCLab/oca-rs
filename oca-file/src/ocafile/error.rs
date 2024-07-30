use oca_file::ocafile::error::ParseError as SemanticsParseError;
use oca_file_transformation::ocafile::error::ParseError as TransformationParseError;

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

    #[error("Error parsing semantics: {0}")]
    SemanticsError(#[from] SemanticsParseError),

    #[error("Error parsing transformation: {0}")]
    TransformationError(#[from] TransformationParseError),

    #[error("{0}")]
    Custom(String),
}
