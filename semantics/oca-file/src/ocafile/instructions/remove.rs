use crate::ocafile::{error::InstructionError, Pair, Rule};
use indexmap::IndexMap;
use log::debug;
use oca_ast::ast::{
    CaptureContent, Command, CommandType, Content, NestedAttrType, NestedValue, ObjectKind,
    OverlayType,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RemoveInstruction {}

impl RemoveInstruction {
    pub(crate) fn from_record(record: Pair, _index: usize) -> Result<Command, InstructionError> {
        let mut object_kind = None;

        debug!("Parsing remove instruction: {:?}", record);
        for object in record.into_inner() {
            match object.as_rule() {
                Rule::remove_meta => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Meta,
                        Content {
                            properties: Some(extract_properties_pairs(object)),
                            attributes: None,
                        },
                    ));
                }
                Rule::remove_classification => {
                    let mut properties: IndexMap<String, NestedValue> = IndexMap::new();
                    properties.insert(
                        "classification".to_string(),
                        NestedValue::Value("".to_string()),
                    );
                    object_kind = Some(ObjectKind::CaptureBase(CaptureContent {
                        attributes: None,
                        properties: Some(properties),
                        flagged_attributes: None,
                    }));
                }
                Rule::remove_label => {
                    object_kind = Some(ObjectKind::Overlay(
                        OverlayType::Label,
                        Content {
                            properties: Some(extract_properties_pairs(object.clone())),
                            attributes: Some(extract_attribute_pairs(object)),
                        },
                    ));
                }
                Rule::remove_attribute => {
                    let mut attributes: IndexMap<String, NestedAttrType> = IndexMap::new();
                    for key in object.into_inner() {
                        debug!("Parsing key to remove: {:?}", key.as_str());
                        attributes.insert(key.as_str().to_string(), NestedAttrType::Null);
                    }
                    object_kind = Some(ObjectKind::CaptureBase(CaptureContent {
                        attributes: Some(attributes),
                        properties: None,
                        flagged_attributes: None,
                    }));
                }
                _ => {
                    return Err(InstructionError::UnexpectedToken(format!(
                        "unexpected token {:?}",
                        object.as_rule()
                    )))
                }
            }
        }

        Ok(Command {
            kind: CommandType::Remove,
            object_kind: object_kind.unwrap(),
        })
    }
}

fn extract_properties_pairs(object: Pair) -> IndexMap<String, NestedValue> {
    let mut properties: IndexMap<String, NestedValue> = IndexMap::new();
    for attr_pairs in object.into_inner() {
        match attr_pairs.as_rule() {
            Rule::prop_key => {
                debug!("Parsed attribute: {:?}", attr_pairs);
                // TODO find out how to parse nested objects
                properties.insert(
                    attr_pairs.as_str().to_string(),
                    NestedValue::Value("".to_string()),
                );
            }
            _ => {
                return properties;
            }
        }
    }
    properties
}

fn extract_attribute_pairs(object: Pair) -> IndexMap<String, NestedValue> {
    let mut attributes: IndexMap<String, NestedValue> = IndexMap::new();
    for attr_pairs in object.into_inner() {
        match attr_pairs.as_rule() {
            Rule::attr_key => {
                debug!("Parsed attribute: {:?}", attr_pairs);
                // TODO find out how to parse nested objects
                attributes.insert(
                    attr_pairs.as_str().to_string(),
                    NestedValue::Value("".to_string()),
                );
            }
            _ => {
                return attributes;
            }
        }
    }
    attributes
}
