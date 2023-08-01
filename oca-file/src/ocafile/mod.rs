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

pub fn parse_from_string(unparsed_file: String) -> Result<OCAAst, String> {
    let file = OCAfileParser::parse(Rule::file, &unparsed_file)
        .map_err(|e| e.to_string())?
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
                    return Err(format!("Error validating instruction: {}", e));
                }
            },
            Err(e) => {
                return Err(format!("Error parsing instruction: {}", e));
            }
        };
    }
    Ok(oca_ast)
}
