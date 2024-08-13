use crate::ocafile::{error::InstructionError, Pair, Rule};
use log::debug;
use oca_ast_semantics::ast::{BundleContent, Command, CommandType, ObjectKind, RefValue, ReferenceAttrType};
use said::SelfAddressingIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FromInstruction {}

impl FromInstruction {
    pub(crate) fn from_record(record: Pair, _index: usize) -> Result<Command, InstructionError> {
        let mut said_pair = None;

        for field in record.into_inner() {
            match field.as_rule() {
                Rule::from_said => said_pair = Some(field),
                Rule::comment => continue,
                _ => {
                    return Err(InstructionError::UnexpectedToken(format!(
                        "unexpected token {:?}",
                        field.as_rule()
                    )))
                }
            };
        }

        let said_str = said_pair.unwrap().as_str();
        let said: SelfAddressingIdentifier = said_str
            .parse()
            .map_err(|_| InstructionError::Parser(format!("Invalid said: {said_str}")))?;
        debug!("Using oca bundle from: {:?}", said);
        let said = ReferenceAttrType::Reference(RefValue::Said(said));
        Ok(Command {
            kind: CommandType::From,
            object_kind: ObjectKind::OCABundle(BundleContent { said }),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ocafile::{self, error::InstructionError, OCAfileParser, Pair, Rule};
    use oca_ast_semantics::ast::RefValue;
    use pest::Parser;

    pub fn parse_direct<T, F>(input: &str, rule: Rule, func: F) -> Result<T, InstructionError>
    where
        F: Fn(Pair) -> Result<T, InstructionError>,
    {
        let pair = OCAfileParser::parse(rule, input)
            .expect("unsuccessful parse")
            .next()
            .ok_or(InstructionError::UnexpectedToken(
                "Unknown parser error".to_string(),
            ))?;

        func(pair)
    }

    use super::*;

    #[test]
    fn test_from_instruction() -> Result<(), InstructionError> {
        // test vector with example instruction and boolean if they should be valid or not
        let instructions = vec![
            ("FROM ENmwqnqVxonf_bNZ0hMipOJJY25dxlC8eSY5BbyMCfLJ", true),
            ("from ENmwqnqVxonf_bNZ0hMipOJJY25dxlC8eSY5BbyMCfLJ", true),
            ("from error", false),
            (
                "from https://humancolossus.org/ENmwqnqVxonf_bNZ0hMipOJJY25dxlC8eSY5BbyMCfLJ",
                false,
            ),
        ];

        for (instruction, is_valid) in instructions {
            let result = parse_direct(instruction, Rule::from, |p| {
                FromInstruction::from_record(p, 0)
            });

            match result {
                Ok(command) => {
                    let content = command.object_kind.oca_bundle_content().unwrap();

                    match content.clone().said {
                        ocafile::ast::ReferenceAttrType::Reference(refs) => match refs {
                            RefValue::Said(_said) => {
                                assert!(is_valid, "Instruction should be valid");
                            }
                            RefValue::Name(_) => {
                                panic!("Refn not supported");
                            }
                        },
                    }
                }
                Err(_e) => {
                    assert!(!is_valid, "Instruction should be invalid")
                }
            }
        }
        let _from = parse_direct(
            "from ENmwqnqVxonf_bNZ0hMipOJJY25dxlC8eSY5BbyMCfLJ",
            Rule::from,
            |p| FromInstruction::from_record(p, 0),
        )?;

        Ok(())
    }
}
