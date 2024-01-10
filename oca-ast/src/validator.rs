use crate::{
    ast::{Command, CommandType, NestedAttrType, NestedValue, OCAAst, ObjectKind},
    errors::Error,
};
use indexmap::{indexmap, IndexMap};
use log::debug;

type CaptureAttributes = IndexMap<String, NestedAttrType>;

/// Validates given commands against existing valid OCA AST
///
/// # Arguments
/// * `ast` - valid OCA AST
/// * `command` - Command to validate against AST
///
/// # Returns
/// * `Result<bool, Error>` - Result of validation
pub trait Validator {
    fn validate(&self, ast: &OCAAst, command: Command) -> Result<bool, Error>;
}

pub struct OCAValidator {}

impl Validator for OCAValidator {
    fn validate(&self, ast: &OCAAst, command: Command) -> Result<bool, Error> {
        let mut errors = Vec::new();
        let mut valid = true;
        match ast.version.as_str() {
            "1.0.0" => {
                let version_validator = validate_1_0_0(ast, command);
                if version_validator.is_err() {
                    valid = false;
                    errors.push(version_validator.err().unwrap());
                }
            }
            "" => {
                valid = false;
                errors.push(Error::MissingVersion());
            }
            _ => {
                valid = false;
                errors.push(Error::InvalidVersion(ast.version.to_string()));
            }
        }
        if valid {
            Ok(true)
        } else {
            Err(Error::Validation(errors))
        }
    }
}

fn validate_1_0_0(ast: &OCAAst, command: Command) -> Result<bool, Error> {
    // Rules
    // Cannot remove if does not exist on stack
    // Cannot modify if does not exist on stack
    // Cannot add if already exists on stack
    // Attributes must have valid type
    let mut valid = true;
    let mut errors = Vec::new();
    match (&command.kind, &command.object_kind) {
        (CommandType::Add, ObjectKind::CaptureBase(_)) => {
            match rule_add_attr_if_not_exist(ast, command) {
                Ok(result) => {
                    if !result {
                        valid = result;
                    }
                }
                Err(error) => {
                    valid = false;
                    errors.push(error);
                }
            }
        }
        (CommandType::Remove, ObjectKind::CaptureBase(_)) => {
            match rule_remove_attr_if_exist(ast, command) {
                Ok(result) => {
                    if !result {
                        valid = result;
                    }
                }
                Err(error) => {
                    valid = false;
                    errors.push(error);
                }
            }
        }

        _ => {
            // TODO: Add support for FROM, MODIFY with combination of different object kinds
        }
    }
    // CommandType::Modify => {
    //     match rule_modify_if_exist(ast, command) {
    //         Ok(result) => {
    //             if !result {
    //                 valid = result;
    //             }
    //         }
    //         Err(error) => {
    //             valid = false;
    //             errors.push(error);
    //         }
    //     }
    // }

    if valid {
        Ok(true)
    } else {
        Err(Error::Validation(errors))
    }
}

/// Check rule for remove command
/// Rule would be valid if attributes which commands tries to remove exist in the stack
///
/// # Arguments
/// * `ast` - valid OCA AST
/// * `command` - Command to validate against AST
///
/// # Returns
/// * `Result<bool, Error>` - Result of validation
fn rule_remove_attr_if_exist(ast: &OCAAst, command_to_validate: Command) -> Result<bool, Error> {
    let mut errors = Vec::new();

    let attributes = extract_attributes(ast);
    let properties = extract_properties(ast);

    let content = command_to_validate.object_kind.capture_content();

    println!("attributes: {:?}", attributes);
    println!("properties: {:?}", properties);

    match (
        content,
        content.as_ref().and_then(|c| c.attributes.as_ref()),
    ) {
        (Some(_content), Some(attrs_to_remove)) => {
            println!("attr to remove: {:?}", attrs_to_remove);
            let valid = attrs_to_remove
                .keys()
                .all(|key| attributes.contains_key(key));
            if !valid {
                errors.push(Error::InvalidOperation(
                    "Cannot remove attribute if does not exists".to_string(),
                ));
            }
        }
        (None, None) => (),
        (None, Some(_)) => (),
        (Some(_), None) => (),
    }

    match (
        content,
        content.as_ref().and_then(|c| c.properties.as_ref()),
    ) {
        (Some(_content), Some(props_to_remove)) => {
            let valid = props_to_remove
                .keys()
                .all(|key| properties.contains_key(key));
            if !valid {
                errors.push(Error::InvalidOperation(
                    "Cannot remove property if does not exists".to_string(),
                ));
                return Err(Error::Validation(errors));
            }
        }
        (None, None) => (),
        (None, Some(_)) => (),
        (Some(_), None) => (),
    }
    if errors.is_empty() {
        Ok(true)
    } else {
        Err(Error::Validation(errors))
    }
}

/// Check rule for add command
/// Rule would be valid if attributes which commands tries to add do not exist in the stack
///
/// # Arguments
/// * `ast` - valid OCA AST
/// * `command` - Command to validate against AST
///
/// # Returns
/// * `Result<bool, Error>` - Result of validation
fn rule_add_attr_if_not_exist(ast: &OCAAst, command_to_validate: Command) -> Result<bool, Error> {
    let mut errors = Vec::new();
    // Create a list of all attributes ADDed and REMOVEd via commands and check if what left covers needs of new command
    let default_attrs: IndexMap<String, NestedAttrType> = indexmap! {};

    let attributes = extract_attributes(ast);

    let content = command_to_validate.object_kind.capture_content();

    match content {
        Some(content) => {
            let attrs_to_add = content.attributes.clone().unwrap_or(default_attrs);
            debug!("attrs_to_add: {:?}", attrs_to_add);

            let existing_keys: Vec<_> = attrs_to_add
                .keys()
                .filter(|key| attributes.contains_key(*key))
                .collect();

            if !existing_keys.is_empty() {
                errors.push(Error::InvalidOperation(format!(
                    "Cannot add attribute if already exists: {:?}",
                    existing_keys
                )));
                Err(Error::Validation(errors))
            } else {
                Ok(true)
            }
        }
        None => {
            errors.push(Error::InvalidOperation(
                "No attribtues specify to be added".to_string(),
            ));
            Err(Error::Validation(errors))
        }
    }
}

fn extract_attributes(ast: &OCAAst) -> CaptureAttributes {
    let default_attrs: IndexMap<String, NestedAttrType> = indexmap! {};
    let mut attributes: CaptureAttributes = indexmap! {};
    for instruction in &ast.commands {
        match (instruction.kind.clone(), instruction.object_kind.clone()) {
            (CommandType::Remove, ObjectKind::CaptureBase(capture_content)) => {
                let attrs = capture_content
                    .attributes
                    .as_ref()
                    .unwrap_or(&default_attrs);
                attributes.retain(|key, _value| !attrs.contains_key(key));
            }
            (CommandType::Add, ObjectKind::CaptureBase(capture_content)) => {
                let attrs = capture_content
                    .attributes
                    .as_ref()
                    .unwrap_or(&default_attrs);
                attributes.extend(attrs.iter().map(|(k, v)| (k.clone(), v.clone())));
            }
            _ => {}
        }
    }
    attributes
}

fn extract_properties(ast: &OCAAst) -> IndexMap<String, NestedValue> {
    let default_attrs: IndexMap<String, NestedValue> = indexmap! {};
    let mut properties: IndexMap<String, NestedValue> = indexmap! {};
    for instruction in &ast.commands {
        match (instruction.kind.clone(), instruction.object_kind.clone()) {
            (CommandType::Remove, ObjectKind::CaptureBase(capture_content)) => {
                let props = capture_content
                    .properties
                    .as_ref()
                    .unwrap_or(&default_attrs);
                properties.retain(|key, _value| !props.contains_key(key));
            }
            (CommandType::Add, ObjectKind::CaptureBase(capture_content)) => {
                let props = capture_content
                    .properties
                    .as_ref()
                    .unwrap_or(&default_attrs);
                properties.extend(props.iter().map(|(k, v)| (k.clone(), v.clone())));
            }
            _ => {}
        }
    }
    properties
}

#[cfg(test)]
mod tests {
    use indexmap::indexmap;

    use super::*;
    use crate::ast::{
        AttributeType, CaptureContent, Command, CommandType, NestedValue, OCAAst, ObjectKind,
    };

    #[test]
    fn test_rule_remove_if_exist() {
        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "documentType".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "photo".to_string() => NestedAttrType::Value(AttributeType::Binary),
                }),
                properties: Some(indexmap! {
                    "classification".to_string() => NestedValue::Value("GICS:1234".to_string()),
                }),
                flagged_attributes: None,
            }),
        };

        let command2 = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(indexmap! {
                    "issuer".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "last_name".to_string() => NestedAttrType::Value(AttributeType::Binary),
                }),
                properties: Some(indexmap! {
                    "classification".to_string() => NestedValue::Value("GICS:1234".to_string()),
                }),
                flagged_attributes: None,
            }),
        };

        let remove_command = Command {
            kind: CommandType::Remove,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedAttrType::Null,
                    "documentType".to_string() => NestedAttrType::Null,
                }),
                properties: Some(indexmap! {}),
                flagged_attributes: None,
            }),
        };

        let add_command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedAttrType::Value(AttributeType::Text),
                }),
                properties: Some(indexmap! {}),
                flagged_attributes: None,
            }),
        };

        let valid_command = Command {
            kind: CommandType::Remove,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedAttrType::Null,
                    "issuer".to_string() => NestedAttrType::Null,
                }),
                properties: Some(indexmap! {}),
                flagged_attributes: None,
            }),
        };

        let invalid_command = Command {
            kind: CommandType::Remove,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(indexmap! {
                    "documentType".to_string() => NestedAttrType::Null,
                }),
                properties: Some(indexmap! {}),
                flagged_attributes: None,
            }),
        };

        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        ocaast.commands.push(command2);
        ocaast.commands.push(remove_command);
        ocaast.commands.push(add_command);
        let mut result = rule_remove_attr_if_exist(&ocaast, valid_command.clone());
        assert!(result.is_ok());
        ocaast.commands.push(invalid_command.clone());
        result = rule_remove_attr_if_exist(&ocaast, invalid_command);
        assert!(result.is_err());
    }

    #[test]
    fn test_rule_add_if_not_exist() {
        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "documentType".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "photo".to_string() => NestedAttrType::Value(AttributeType::Binary),
                }),
                properties: Some(indexmap! {
                    "classification".to_string() => NestedValue::Value("GICS:1234".to_string()),
                }),
                flagged_attributes: None,
            }),
        };

        let command2 = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(indexmap! {
                    "issuer".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "last_name".to_string() => NestedAttrType::Value(AttributeType::Binary),
                }),
                properties: Some(indexmap! {}),
                flagged_attributes: None,
            }),
        };

        let valid_command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(indexmap! {
                    "first_name".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "address".to_string() => NestedAttrType::Value(AttributeType::Text),
                }),
                properties: Some(indexmap! {}),
                flagged_attributes: None,
            }),
        };

        let invalid_command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "phone".to_string() => NestedAttrType::Value(AttributeType::Text),
                }),
                properties: Some(indexmap! {}),
                flagged_attributes: None,
            }),
        };

        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        ocaast.commands.push(command2);
        let mut result = rule_add_attr_if_not_exist(&ocaast, valid_command.clone());
        assert!(result.is_ok());
        ocaast.commands.push(invalid_command.clone());
        result = rule_add_attr_if_not_exist(&ocaast, invalid_command.clone());
        assert!(result.is_err());
    }
}
