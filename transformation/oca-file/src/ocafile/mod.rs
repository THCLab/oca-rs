pub mod error;
mod instructions;

use self::{error::ParseError, instructions::{rename::RenameInstruction, link::LinkInstruction}};
use crate::ocafile::error::InstructionError;
use oca_ast_transformation::{
    ast::{
        self, Command, CommandMeta
    },
    validator::{OCAValidator, Validator},
};
pub use oca_ast_transformation::ast::TransformationAST;
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
    type Error = InstructionError;
    fn try_from_pair(record: Pair) -> std::result::Result<Self, Self::Error> {
        let instruction: Command = match record.as_rule() {
            Rule::rename => RenameInstruction::from_record(record, 0)?,
            Rule::link => LinkInstruction::from_record(record, 0)?,
            _ => {
                return Err(InstructionError::UnexpectedToken(
                    record.to_string(),
                ))
            }
        };
        Ok(instruction)
    }
}

pub fn parse_from_string(unparsed_file: String) -> Result<TransformationAST, ParseError> {
    let file = OCAfileParser::parse(Rule::file, &unparsed_file)
        .map_err(|e| {
            let (line_number, column_number) = match e.line_col {
                pest::error::LineColLocation::Pos((line, column)) => {
                    (line, column)
                }
                pest::error::LineColLocation::Span((line, column), _) => {
                    (line, column)
                }
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

    let mut oca_ast = TransformationAST::new();

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
                        return Err(ParseError::MetaError(
                            attr.as_str().to_string(),
                        ));
                    }
                }
            }
            if key.is_empty() {
                return Err(ParseError::MetaError("key is empty".to_string()));
            }
            if value.is_empty() {
                return Err(ParseError::MetaError(
                    "value is empty".to_string(),
                ));
            }
            oca_ast.meta.insert(key, value);
            continue;
        }
        if let Rule::empty_line = line.as_rule() {
            continue;
        }

        match Command::try_from_pair(line.clone()) {
            Ok(command) => {
                match validator.validate(&oca_ast, command.clone()) {
                    Ok(_) => {
                        oca_ast.commands.push(command);
                        oca_ast.commands_meta.insert(
                            oca_ast.commands.len() - 1,
                            CommandMeta {
                                line_number: n + 1,
                                raw_line: line.as_str().to_string(),
                            },
                        );
                    }
                    Err(e) => {
                        return Err(ParseError::Custom(format!(
                            "Error validating instruction: {}",
                            e
                        )));
                    }
                }
            }
            Err(e) => {
                return Err(ParseError::InstructionError(e));
            }
        };
    }
    Ok(oca_ast)
}

pub fn generate_from_ast(ast: &TransformationAST) -> String {
    let ocafile = String::new();

    ast.commands.iter().for_each(|command| {
        let _line = String::new();

        match command.kind {
            ast::CommandType::Rename => todo!(),
            ast::CommandType::Link => todo!(),
        }

        // ocafile.push_str(format!("{}\n", line).as_str());
    });

    ocafile
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from_string_valid() {
        let _ = env_logger::builder().is_test(true).try_init();

        let unparsed_file = r#"
-- version=0.0.1
LINK ATTRIBUTE surname -> last_name
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string()).unwrap();
        println!("{:#?}", oca_ast);
        assert_eq!(oca_ast.meta.get("version").unwrap(), "0.0.1");
    }
}
