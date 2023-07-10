use crate::ocafile::{error::Error, Pair, Rule};
use indexmap::IndexMap;
use log::debug;
use oca_ast::ast::{Command, CommandType, Content, NestedValue, ObjectKind, OverlayType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RemoveInstruction {}

impl RemoveInstruction {
    pub(crate) fn from_record(record: Pair, _index: usize) -> Result<Command, Error> {
        let mut object_kind = None;
        let mut content = None;

        debug!("Parsing remove instruction: {:?}", record);
        for object in record.into_inner() {
            content = match object.as_rule() {
                Rule::remove_meta => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::Meta));
                    extract_content(object)
                }
                Rule::classification => {
                    object_kind = Some(ObjectKind::CaptureBase);
                    let mut properties: IndexMap<String, NestedValue> = IndexMap::new();
                    properties.insert(
                        "classification".to_string(),
                        NestedValue::Value("".to_string()),
                    );
                    Some(Content {
                        properties: Some(properties),
                        attributes: None,
                    })
                }
                Rule::remove_label => {
                    object_kind = Some(ObjectKind::Overlay(OverlayType::Label));
                    extract_content(object)
                }
                Rule::remove_attribute => {
                    object_kind = Some(ObjectKind::CaptureBase);
                    let mut attributes: IndexMap<String, NestedValue> = IndexMap::new();
                    for key in object.into_inner() {
                        debug!("Parsing key to remove: {:?}", key.as_str());
                        attributes
                            .insert(key.as_str().to_string(), NestedValue::Value("".to_string()));
                    }
                    Some(Content {
                        properties: None,
                        attributes: Some(attributes),
                    })
                }
                _ => {
                    return Err(Error::UnexpectedToken(format!(
                        "unexpected token {:?}",
                        object.as_rule()
                    )))
                }
            }
        }

        Ok(Command {
            kind: CommandType::Remove,
            object_kind: object_kind.unwrap(),
            content,
        })
    }
}


// Extract content from instruction for ADD and MODIFY command

fn extract_content(object: Pair) -> Option<Content> {
    let mut properties: IndexMap<String, NestedValue> = IndexMap::new();
    let mut attributes: IndexMap<String, NestedValue> = IndexMap::new();

    debug!("Into the object: {:?}", object);
    for attr in object.into_inner() {
        debug!("Inside object: {:?}", attr);
        match attr.as_rule() {
            Rule::attr_key => {
                debug!("Parsed attribute: {:?}", attr);
                // TODO find out how to parse nested objects
                attributes.insert(
                    attr.as_str().to_string(),
                    NestedValue::Value("".to_string()),
                );
            }
            Rule::prop_key => {
                debug!("Parsed attribute: {:?}", attr);
                // TODO find out how to parse nested objects
                properties.insert(
                    attr.as_str().to_string(),
                    NestedValue::Value("".to_string()),
                );
            }
            Rule::lang => {
                debug!("Parsing language: {:?}", attr.as_str());
                properties.insert(
                    "lang".to_string(),
                    NestedValue::Value(attr.as_str().to_string()),
                );
            }
            _ => {
                debug!(
                    "Unexpected token: Invalid attribute in instruction {:?}",
                    attr.as_rule()
                );
                return None;
            }
        }
    }
    debug!("Parsed properties: {:?}", properties);
    debug!("Parsed attributes: {:?}", attributes);
    Some(Content {
        properties: Some(properties),
        attributes: Some(attributes),
    })
}
