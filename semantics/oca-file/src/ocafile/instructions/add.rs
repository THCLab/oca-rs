use crate::ocafile::{error::InstructionError, instructions::helpers, Pair, Rule};
use indexmap::IndexMap;
use log::{debug, info};
use oca_ast_semantics::ast::{
    CaptureContent, Command, CommandType, NestedAttrType, NestedValue, ObjectKind, OverlayType,
};

pub struct AddInstruction {}

impl AddInstruction {
    pub(crate) fn from_record(record: Pair, _index: usize) -> Result<Command, InstructionError> {
        let mut object_kind = None;
        let kind = CommandType::Add;

        debug!("Parsing add instruction from the record: {:?}", record);
        for object in record.into_inner() {
            let overlay_version = "1.1".to_string();
            match object.as_rule() {
                Rule::meta => {
                    object_kind = Some(ObjectKind::Overlay(
                        oca_ast_semantics::ast::OverlayType::Meta(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::attribute => {
                    let mut attributes: IndexMap<String, NestedAttrType> = IndexMap::new();
                    for attr_pairs in object.into_inner() {
                        match attr_pairs.as_rule() {
                            Rule::attr_pairs => {
                                debug!("Attribute pairs: {:?}", attr_pairs);
                                for attr in attr_pairs.into_inner() {
                                    debug!("Parsing attribute pair {:?}", attr);
                                    let (key, value) = helpers::extract_attribute(attr)?;
                                    info!("Parsed attribute: {:?} = {:?}", key, value);

                                    attributes.insert(key, value);
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
                    debug!("Attributes: {:?}", attributes);
                    object_kind = Some(ObjectKind::CaptureBase(CaptureContent {
                        properties: None,
                        attributes: Some(attributes),
                        flagged_attributes: None,
                    }));
                }
                Rule::comment => continue,
                Rule::classification => {
                    let mut properties: IndexMap<String, NestedValue> = IndexMap::new();
                    let classification = object.into_inner().next().unwrap();
                    print!("Classification: {:?}", classification.as_rule());
                    properties.insert(
                        "classification".to_string(),
                        NestedValue::Value(classification.as_str().to_string()),
                    );
                    object_kind = Some(ObjectKind::CaptureBase(CaptureContent {
                        properties: Some(properties),
                        attributes: None,
                        flagged_attributes: None,
                    }));
                }
                Rule::information => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Information(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::character_encoding => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::CharacterEncoding(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::character_encoding_props => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::CharacterEncoding(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::label => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Label(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::unit => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Unit(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::format => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Format(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::conformance => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Conformance(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::conditional => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Conditional(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::cardinality => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Cardinality(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::entry_code => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::EntryCode(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::entry => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Entry(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::link => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Link(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::attribute_framing => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::AttributeFraming(overlay_version),
                        helpers::extract_content(object),
                    ));
                }
                Rule::flagged_attrs => {
                    object_kind = Some(ObjectKind::CaptureBase(CaptureContent {
                        properties: None,
                        attributes: None,
                        flagged_attributes: Some(helpers::extract_flagged_attrs(object)),
                    }));
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
    fn test_add_attribute_instruction() {
        // test vector with example instruction and boolean if they should be valid or not
        let instructions = vec![
            ("ADD ATTRIBUTE documentNumber = Array[refn:dokument]", true),
            ("ADD ATTRIBUTE documentNumber=Array[Array[Array[refn:dokument]]]", true),
            ("ADD ATTRIBUTE documentNumber=Array[ refs:ENyO7FUBx7oILUYt8FwmLaDVmvOZGETXWHICultMSEpW ]", true),
            ("ADD ATTRIBUTE documentNumber=Array[refn:klient, refs:ENyO7FUBx7oILUYt8FwmLaDVmvOZGETXWHICultMSEpW]", false),
            ("ADD ATTRIBUTE documentNumber=snieg documentType=refs:ENyO7FUBx7oILUYt8FwmLaDVmvOZGETXWHICultMSEpW", false),
            ("ADD ATTRIBUTE documentNumber=refn:snieg documentType=refs:ENyO7FUBx7oILUYt8FwmLaDVmvOZGETXWHICultMSEpW", true),
            ("ADD ATTRIBUTE documentNumber=Text documentType=Numeric", true),
            ("ADD ATTRIBUTE documentNumber=Text documentType=Numeric name=Text list=Array[Numeric]", true),
            ("ADD ATTRIBUTE name=Text", false),
            ("ADD ATTR name=Text", false),
            ("ADD attribute name=Text", true),
            ("add attribute name=Text", true),
            ("add attribute name=Random", false),
            ("add flagged_attributes name documentType", true),
        ];
        let _ = env_logger::builder().is_test(true).try_init();

        // loop over instructions to check if the are meeting the requirements
        for (instruction, is_valid) in instructions {
            debug!("Instruction: {:?}", instruction);
            let parsed_instruction = OCAfileParser::parse(Rule::add, instruction);
            debug!("Parsed instruction: {:?}", parsed_instruction);

            match parsed_instruction {
                Ok(mut parsed_instruction) => {
                    let instruction = parsed_instruction.next();
                    assert!(instruction.is_some());
                    match instruction {
                        Some(instruction) => {
                            let instruction = AddInstruction::from_record(instruction, 0).unwrap();

                            assert_eq!(instruction.kind, CommandType::Add);
                            match instruction.object_kind {
                                ObjectKind::CaptureBase(content) => {
                                    if content.attributes.is_some() {
                                        assert!(!content.attributes.unwrap().is_empty());
                                    }
                                    if content.properties.is_some() {
                                        assert!(!content.properties.unwrap().is_empty());
                                    }
                                    if content.flagged_attributes.is_some() {
                                        assert!(!content.flagged_attributes.unwrap().is_empty());
                                    }
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

    #[test]
    fn test_add_overlay_instructions() {
        let instructions = vec![
            ("ADD ENTRY_CODE ATTRS radio=[\"o1\",\"o2\", \"o3\"]", true),
            ("ADD FORMAT ATTRS name = \"^\\d+$\"", true),
            ("ADD CHARACTER_ENCODING ATTRS name=\"utf-16le\"", true),
        ];

        let _ = env_logger::builder().is_test(true).try_init();

        for (instruction, is_valid) in instructions {
            debug!("Instruction: {:?}", instruction);
            let parsed_instruction = OCAfileParser::parse(Rule::add, instruction);
            debug!("Parsed instruction: {:?}", parsed_instruction);

            match parsed_instruction {
                Ok(mut parsed_instruction) => {
                    let instruction = parsed_instruction.next();
                    assert!(instruction.is_some());
                    match instruction {
                        Some(instruction) => {
                            let instruction = AddInstruction::from_record(instruction, 0).unwrap();
                            println!("Instruction: {:?}", instruction);

                            assert_eq!(instruction.kind, CommandType::Add);
                            match instruction.object_kind {
                                ObjectKind::Overlay(overlay_type, content) => match overlay_type {
                                    OverlayType::EntryCode(_) => {
                                        let attr_array = NestedValue::Array(vec![
                                            NestedValue::Value("o1".to_string()),
                                            NestedValue::Value("o2".to_string()),
                                            NestedValue::Value("o3".to_string()),
                                        ]);

                                        assert_eq!(
                                            content.attributes.unwrap().get("radio").unwrap(),
                                            &attr_array
                                        );
                                    }
                                    OverlayType::Format(_) => {
                                        let attr_array = NestedValue::Value("^\\d+$".to_string());

                                        assert_eq!(
                                            content.attributes.unwrap().get("name").unwrap(),
                                            &attr_array
                                        );
                                    }
                                    OverlayType::CharacterEncoding(_) => {
                                        let attr_array = NestedValue::Value("utf-16le".to_string());

                                        assert_eq!(
                                            content.attributes.unwrap().get("name").unwrap(),
                                            &attr_array
                                        );
                                    }
                                    _ => {
                                        assert!(!is_valid, "Instruction is not valid");
                                    }
                                },
                                ObjectKind::CaptureBase(_) => todo!(),
                                ObjectKind::OCABundle(_) => todo!(),
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
