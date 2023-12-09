use crate::ocafile::{error::Error, Pair, Rule};
use indexmap::IndexMap;
use log::debug;
use oca_ast::ast::{Command, CommandType, Content, NestedValue, ObjectKind};
use said::SelfAddressingIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FromInstruction {}

impl FromInstruction {
    pub(crate) fn from_record(record: Pair, _index: usize) -> Result<Command, Error> {
        let mut said_pair = None;

        for field in record.into_inner() {
            match field.as_rule() {
                Rule::from_said => said_pair = Some(field),
                Rule::comment => continue,
                _ => {
                    return Err(Error::UnexpectedToken(format!(
                        "unexpected token {:?}",
                        field.as_rule()
                    )))
                }
            };
        }

        let said_str = said_pair.unwrap().as_str();
        let said: SelfAddressingIdentifier = said_str.parse()
            .map_err(|_| Error::Parser(format!("Invalid said: {said_str}")))?;
        debug!("Using oca bundle from: {:?}", said);
        let mut properties: IndexMap<String, NestedValue> = IndexMap::new();
        properties.insert("said".to_string(), NestedValue::Value(said.to_string()));
        Ok(Command {
            kind: CommandType::From,
            object_kind: ObjectKind::OCABundle(Content {
                properties: Some(properties),
                attributes: None,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ocafile::{error::Error, OCAfileParser, Pair, Rule};
    use pest::Parser;
    use std::str::FromStr;

    pub fn parse_direct<T, F>(input: &str, rule: Rule, func: F) -> Result<T, Error>
    where
        F: Fn(Pair) -> Result<T, Error>,
    {
        let pair = OCAfileParser::parse(rule, input)
            .expect("unsuccessful parse")
            .next()
            .ok_or(Error::UnexpectedToken("Unknown parser error".to_string()))?;

        func(pair)
    }

    use super::*;

    #[test]
    fn test_from_instruction() -> Result<(), Error> {
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
                    let properties = content.properties.as_ref().unwrap();
                    let said_value = properties.get("said").unwrap();
                    match said_value {
                        // TODO this should be simple reference not value
                        NestedValue::Value(said_str) => {
                            SelfAddressingIdentifier::from_str(said_str).unwrap();
                            assert!(is_valid, "Instruction should be valid");
                        }
                        _ => {
                            panic!("said should be a value");
                        }
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
