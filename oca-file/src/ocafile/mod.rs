mod error;
mod instructions;

use self::instructions::{add::AddInstruction, from::FromInstruction, remove::RemoveInstruction};
use crate::ocafile::error::Error;
use oca_ast::{
    ast::{Command, CommandMeta, OCAAst},
    validator::{OCAValidator, Validator},
};
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "ocafile.pest"]
pub struct OCAfileParser;

pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub trait TryFromPair {
    type Error;
    fn try_from_pair(pair: Pair<'_>) -> Result<Command, Self::Error>;
}

impl TryFromPair for Command {
    type Error = Error;
    fn try_from_pair(record: Pair) -> std::result::Result<Self, Self::Error> {
        let instruction: Command = match record.as_rule() {
            Rule::from => FromInstruction::from_record(record, 0)?,
            Rule::add => AddInstruction::from_record(record, 0)?,
            Rule::remove => RemoveInstruction::from_record(record, 0)?,
            _ => return Err(Error::UnexpectedToken(record.to_string())),
        };
        Ok(instruction)
    }
}

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
    #[error("{0}")]
    Custom(String),
}

pub fn parse_from_string(unparsed_file: String) -> Result<OCAAst, ParseError> {
    let file = OCAfileParser::parse(Rule::file, &unparsed_file)
        .map_err(|e| {
            let (line_number, column_number) = match e.line_col {
                pest::error::LineColLocation::Pos((line, column)) => (line, column),
                pest::error::LineColLocation::Span((line, column), _) => (line, column),
            };
            ParseError::GrammarError {
                line_number,
                column_number,
                raw_line: e.line().to_string(),
                message: e.variant.to_string(),
            }
        })?
        .next()
        .unwrap();

    let mut oca_ast = OCAAst::new();

    let validator = OCAValidator {};

    for (n, line) in file.into_inner().enumerate() {
        if let Rule::EOI = line.as_rule() {
            continue;
        }
        if let Rule::comment = line.as_rule() {
            continue;
        }
        if let Rule::meta_comment = line.as_rule() {
            let mut key = "".to_string();
            let mut value = "".to_string();
            for attr in line.into_inner() {
                match attr.as_rule() {
                    Rule::meta_attr_key => {
                        key = attr.as_str().to_string();
                    }
                    Rule::meta_attr_value => {
                        value = attr.as_str().to_string();
                    }
                    _ => {
                        return Err(ParseError::Custom(format!("Error parsing meta: {}", attr.as_str())));
                    }
                }
            }
            if key == "" {
                return Err(ParseError::Custom(format!("Error parsing meta: key is empty")));
            }
            if value == "" {
                return Err(ParseError::Custom(format!("Error parsing meta: value is empty")));
            }
            oca_ast.meta.insert(key, value);
            continue;
        }
        if let Rule::empty_line = line.as_rule() {
            continue;
        }

        match Command::try_from_pair(line.clone()) {
            Ok(command) => match validator.validate(&oca_ast, command.clone()) {
                Ok(_) => {
                    oca_ast.commands.push(command);
                    oca_ast.commands_meta.insert(oca_ast.commands.len() - 1, CommandMeta {
                        line_number: n + 1,
                        raw_line: line.as_str().to_string(),
                    });
                }
                Err(e) => {
                    return Err(ParseError::Custom(format!("Error validating instruction: {}", e)));
                }
            },
            Err(e) => {
                return Err(ParseError::Custom(format!("Error parsing instruction: {}", e)));
            }
        };
    }
    Ok(oca_ast)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_meta_from_string_valid() {
        let unparsed_file = r#"
-- version=0.0.1
-- name=Objekt
ADD attribute name=Text age=Numeric
"#;

        let oca_ast = parse_from_string(unparsed_file.to_string()).unwrap();
        assert_eq!(oca_ast.meta.get("version").unwrap(), "0.0.1");
        assert_eq!(oca_ast.meta.get("name").unwrap(), "Objekt");
    }
}
