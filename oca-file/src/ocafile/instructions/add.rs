use crate::ocafile::{error::Error, instructions::helpers, Pair, Rule};
use indexmap::IndexMap;
use log::{debug, info};
use oca_ast::ast::{Command, CommandType, Content, NestedValue, ObjectKind, OverlayType};

pub struct AddInstruction {}

impl AddInstruction {
    pub(crate) fn from_record(record: Pair, _index: usize) -> Result<Command, Error> {
        let mut object_kind = None;
        let kind = CommandType::Add;
        let mut content = None;

        debug!("Into the record: {:?}", record);
        for object in record.into_inner() {
            content = match object.as_rule() {
                Rule::meta => {
                    object_kind = Some(ObjectKind::Overlay(oca_ast::ast::OverlayType::Meta));
                    helpers::extract_content(object)
                }
                Rule::attribute => {
                    object_kind = Some(ObjectKind::CaptureBase);
                    let mut attributes: IndexMap<String, NestedValue> = IndexMap::new();
                    for attr_pairs in object.into_inner() {
                        match attr_pairs.as_rule() {
                            Rule::attr_pairs => {
                                info!("attribute: {:?}", attr_pairs);
                                for attr in attr_pairs.into_inner() {
                                    debug!("Parsing attribute {:?}", attr);
                                    if let Some((key, value)) =
                                        helpers::extract_attribute_key_pairs(attr)
                                    {
                                        debug!("Parsed attribute: {:?} = {:?}", key, value);

                                        // TODO find out how to parse nested objects
                                        attributes.insert(key, value);
                                    } else {
                                        debug!("Skipping attribute");
                                    }
                                }
                            }
                            _ => {
                                return Err(Error::UnexpectedToken(format!(
                                    "Invalid attributes in ATTRIBUTE instruction {:?}",
                                    attr_pairs.as_rule()
                                )))
                            }
                        }
                    }
                    Some(Content {
                        properties: None,
                        attributes: Some(attributes),
                    })
                }
                Rule::comment => continue,
                Rule::classification => {
                    object_kind = Some(ObjectKind::CaptureBase);
                    let mut properties: IndexMap<String, NestedValue> = IndexMap::new();
                    let classification = object.into_inner().next().unwrap();
                    print!("Classification: {:?}", classification.as_rule());
                    properties.insert(
                        "classification".to_string(),
                        NestedValue::Value(classification.as_str().to_string()),
                    );

                    Some(Content {
                        properties: Some(properties),
                        attributes: None,
                    })
                }
                Rule::information => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::Information));
                    helpers::extract_content(object)
                }
                Rule::character_encoding => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::CharacterEncoding));
                    helpers::extract_content(object)
                }
                Rule::character_encoding_props => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::CharacterEncoding));
                    helpers::extract_content(object)
                }
                Rule::label => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::Label));
                    helpers::extract_content(object)
                }
                Rule::unit => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::Unit));
                    helpers::extract_content(object)
                }
                Rule::format => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::Format));
                    helpers::extract_content(object)
                }
                Rule::conformance => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::Conformance));
                    helpers::extract_content(object)
                }
                Rule::conditional => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::Conditional));
                    helpers::extract_content(object)
                }
                Rule::cardinality => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::Cardinality));
                    helpers::extract_content(object)
                }
                Rule::entry_code => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::EntryCode));
                    helpers::extract_content(object)
                }
                Rule::entry => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::Entry));
                    helpers::extract_content(object)
                }
                Rule::flagged_attrs => {
                    object_kind = Some(ObjectKind::CaptureBase);
                    Some(Content {
                        properties: None,
                        attributes: None,
                    })
                }
                _ => {
                    return Err(Error::UnexpectedToken(format!(
                        "Overlay: unexpected token {:?}",
                        object.as_rule()
                    )))
                }
            };
        }

        Ok(Command {
            kind,
            object_kind: object_kind.unwrap(),
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ocafile::OCAfileParser;

    use super::*;
    use pest::Parser;

    #[test]
    fn test_add_attribute_instruction() {
        // test vector with example instruction and boolean if they should be valid or not
        let instructions = vec![
            ("ADD ATTRIBUTE documentNumber=Text documentType=Numeric", true),
            ("ADD ATTRIBUTE documentNumber=Text documentType=Numeric name=Text list=Array[Numeric]", true),
            ("ADD ATTRIBUTE name=Text", false),
            ("ADD ATTR name=Text", false),
            ("ADD attribute name=Text", true),
            ("add attribute name=Text", true),
            ("add attribute name=Random", false),
        ];

        // loop over instructions to check if the are meeting the requirements
        for (instruction, is_valid) in instructions {
            let parsed_instruction = OCAfileParser::parse(Rule::add, instruction);

            match parsed_instruction {
                Ok(mut parsed_instruction) => {
                    let instruction = parsed_instruction.next();
                    assert!(instruction.is_some());
                    match instruction {
                        Some(instruction) => {
                            let instruction = AddInstruction::from_record(instruction, 0).unwrap();
                            println!("Parsed instruction: {:?}", instruction);

                            assert_eq!(instruction.kind, CommandType::Add);
                            assert_eq!(instruction.object_kind, ObjectKind::CaptureBase);
                            match instruction.content {
                                Some(content) => {
                                    assert!(content.attributes.is_some());
                                    assert!(content.attributes.unwrap().len() > 0);
                                }
                                None => {
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
