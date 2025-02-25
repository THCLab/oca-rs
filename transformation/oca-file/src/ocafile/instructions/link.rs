use crate::ocafile::{error::InstructionError, instructions::helpers, Pair, Rule};
use indexmap::IndexMap;
use log::{debug, info};
use oca_ast_transformation::ast::{Command, CommandType, LinkContent, ObjectKind};

pub struct LinkInstruction {}

impl LinkInstruction {
    pub(crate) fn from_record(record: Pair, _index: usize) -> Result<Command, InstructionError> {
        let mut object_kind = None;
        let kind = CommandType::Link;

        debug!("Parsing link instruction from the record: {:?}", record);
        for object in record.into_inner() {
            match object.as_rule() {
                Rule::link_attributes => {
                    let mut link_attributes: IndexMap<String, String> = IndexMap::new();
                    for attr_pairs in object.into_inner() {
                        match attr_pairs.as_rule() {
                            Rule::link_attr_pairs => {
                                debug!("Attribute pairs: {:?}", attr_pairs);
                                for attr in attr_pairs.into_inner() {
                                    debug!("Parsing attribute pair {:?}", attr);
                                    let (key, value) = helpers::extract_link_attribute(attr)?;
                                    info!("Parsed attribute: {:?} = {:?}", key, value);

                                    link_attributes.insert(key, value);
                                }
                            }
                            _ => {
                                return Err(InstructionError::UnexpectedToken(format!(
                                    "Invalid attributes in ATTRIBUTE instruction {:?}",
                                    attr_pairs.as_rule()
                                )))
                            }
                        }
                    }
                    debug!("Attributes: {:?}", link_attributes);

                    object_kind = Some(ObjectKind::Link(LinkContent {
                        attributes: Some(link_attributes),
                    }));
                    /* object_kind = Some(ObjectKind::CaptureBase(CaptureContent {
                        properties: None,
                        attributes: Some(link_attributes),
                        flagged_attributes: None,
                    })); */
                }
                _ => {
                    return Err(InstructionError::UnexpectedToken(format!(
                        "Overlay: unexpected token {:?}",
                        object.as_rule()
                    )))
                }
            };
        }

        Ok(Command {
            kind,
            object_kind: object_kind.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ocafile::OCAfileParser;

    use super::*;
    use pest::Parser;

    #[test]
    fn test_link_attribute_instruction() {
        // test vector with example instruction and boolean if they should be valid or not
        let instructions = vec![
            ("LINK ATTRIBUTE documentNumber -> document_number", true),
            ("LINK ATTRIBUTE name", false),
        ];
        let _ = env_logger::builder().is_test(true).try_init();

        // loop over instructions to check if the are meeting the requirements
        for (instruction, is_valid) in instructions {
            debug!("Instruction: {:?}", instruction);
            let parsed_instruction = OCAfileParser::parse(Rule::link, instruction);
            debug!("Parsed instruction: {:?}", parsed_instruction);

            match parsed_instruction {
                Ok(mut parsed_instruction) => {
                    let instruction = parsed_instruction.next();
                    assert!(instruction.is_some());
                    match instruction {
                        Some(instruction) => {
                            let instruction = LinkInstruction::from_record(instruction, 0).unwrap();

                            assert_eq!(instruction.kind, CommandType::Link);
                            println!("{:?}", instruction.object_kind);
                            match instruction.object_kind {
                                ObjectKind::Link(link_content) => {
                                    assert_eq!(
                                        link_content.attributes.unwrap().get("documentNumber"),
                                        Some(&"document_number".to_string())
                                    );
                                }
                                _ => {
                                    assert!(!is_valid, "Instruction is not valid");
                                }
                            }
                        }
                        None => {
                            assert!(!is_valid, "Instruction is not valid");
                        }
                    }
                }
                Err(e) => {
                    assert!(!is_valid, "Instruction should be invalid");
                    println!("Error: {:?}", e);
                }
            }
        }
    }
}
