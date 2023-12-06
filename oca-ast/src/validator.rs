use crate::{
    ast::{Command, CommandType, NestedValue, OCAAst, ObjectKind, NestedAttrType},
    errors::Error,
};
use indexmap::{indexmap, IndexMap};
use log::debug;

type PropContentType = IndexMap<String, NestedValue>;
type AttrContentType = IndexMap<String, NestedAttrType>;

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
    match command.kind {
        CommandType::Add => {
            if command.object_kind == ObjectKind::CaptureBase {
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
        }
        CommandType::Remove => {
            if command.object_kind == ObjectKind::CaptureBase {
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
        }
        CommandType::Modify => {
        },
        CommandType::From => {

        },
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
    }

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
    // Create a list of all attributes ADDed and REMOVEd via commands and check if what left covers needs of new command
    let default_attrs: IndexMap<String, NestedAttrType> = indexmap! {};
    let default_props: IndexMap<String, NestedValue> = indexmap! {};

    let (attributes, properties) = extract_content(ast);

    let content = command_to_validate.content.as_ref().unwrap();
    match content {
        crate::ast::CommandContent::AttributeTypes(inner_content) => {
            let attrs_to_remove = inner_content.attributes.clone().unwrap_or(default_attrs);
            debug!("attrs_to_remove: {:?}", attrs_to_remove);
            let valid = attrs_to_remove
                .keys()
                .all(|key| !attributes.contains_key(key));
                if valid {
                    Ok(true)
                } else {
                    errors.push(Error::InvalidOperation(
                    "Cannot add attribute if already exists".to_string(),
                ));
                Err(Error::Validation(errors))
            }
        }
        crate::ast::CommandContent::Properties(inner_prop) => {
            let props_to_remove = inner_prop.properties.clone().unwrap_or(default_props);
            debug!("props_to_remove: {:?}", props_to_remove);
            let valid = props_to_remove
                .keys()
                .all(|key| !properties.contains_key(key));
            if valid {
                Ok(true)
            } else {
                errors.push(Error::InvalidOperation(
                    "Cannot add attribute if already exists".to_string(),
                ));
                Err(Error::Validation(errors))
            }
        }
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
    let default_props: IndexMap<String, NestedValue> = indexmap! {};

    let (attributes, properties) = extract_content(ast);


    let content = command_to_validate.content.as_ref().unwrap();
    match content {
        crate::ast::CommandContent::AttributeTypes(inner_content) => {
            let attrs_to_add = inner_content.attributes.clone().unwrap_or(default_attrs);
            debug!("attrs_to_add: {:?}", attrs_to_add);
            let valid = attrs_to_add
                .keys()
                .all(|key| !attributes.contains_key(key));
                if valid {
                    Ok(true)
                } else {
                    errors.push(Error::InvalidOperation(
                    "Cannot add attribute if already exists".to_string(),
                ));
                Err(Error::Validation(errors))
            }
        }
        crate::ast::CommandContent::Properties(inner_prop) => {
            let props_to_remove = inner_prop.properties.clone().unwrap_or(default_props);
            debug!("props_to_remove: {:?}", props_to_remove);
            let valid = props_to_remove
                .keys()
                .all(|key| !properties.contains_key(key));
            if valid {
                Ok(true)
            } else {
                errors.push(Error::InvalidOperation(
                    "Cannot add attribute if already exists".to_string(),
                ));
                Err(Error::Validation(errors))
            }
        }
    }
}



fn extract_content(ast: &OCAAst) -> (AttrContentType, PropContentType) {
    let default_attrs: IndexMap<String, NestedAttrType> = indexmap! {};
    let default_props: IndexMap<String, NestedValue> = indexmap! {};
    let mut attributes: AttrContentType = indexmap! {};
    let mut properties: PropContentType = indexmap! {};
    // Collect properties and attributes from all commands for purpose of validation
    for instruction in &ast.commands {
        match instruction.kind {
            CommandType::Remove => {
                if instruction.object_kind == ObjectKind::CaptureBase {
                    let content = instruction.content.clone().unwrap();
                    match content {
                        crate::ast::CommandContent::AttributeTypes(inner_content) => {
                            let attrs = inner_content.attributes.as_ref().unwrap_or(&default_attrs);
                            attributes.retain(|key, _value| !attrs.contains_key(key));
                        }
                        crate::ast::CommandContent::Properties(inner_prop) => {
                            let props = inner_prop.properties.as_ref().unwrap_or(&default_props);
                            properties.retain(|key, _value| !props.contains_key(key));
                        }

                    }
                }
            }
            CommandType::Add => {
                if instruction.object_kind == ObjectKind::CaptureBase {
                    let conten = instruction.content.clone().unwrap();
                    match conten {
                        crate::ast::CommandContent::AttributeTypes(inner_content) => {
                            let attrs = inner_content.attributes.as_ref().unwrap_or(&default_attrs);
                            attributes.extend(attrs.iter().map(|(k, v)| (k.clone(), v.clone())));
                        }
                        crate::ast::CommandContent::Properties(inner_prop) => {
                            let props = inner_prop.properties.as_ref().unwrap_or(&default_props);
                            properties.extend(props.iter().map(|(k, v)| (k.clone(), v.clone())));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    (attributes, properties)
}

#[cfg(test)]
mod tests {
    use indexmap::indexmap;

    use super::*;
    use crate::ast::{Command, CommandType, Content, NestedValue, OCAAst, ObjectKind, AttributeType, CommandContent, AttributeTypeContent};

    #[test]
    fn test_rule_remove_if_exist() {
        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase,
            content: Some(CommandContent::AttributeTypes(AttributeTypeContent {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "documentType".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "photo".to_string() => NestedAttrType::Value(AttributeType::Binary),
                }),
            })),
        };

        let command2 = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase,
            content: Some(CommandContent::AttributeTypes( AttributeTypeContent {
                attributes: Some(indexmap! {
                    "issuer".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "last_name".to_string() => NestedAttrType::Value(AttributeType::Binary),
                }),
            })),
        };

        let remove_command = Command {
            kind: CommandType::Remove,
            object_kind: ObjectKind::CaptureBase,
            content: Some( CommandContent::AttributeTypes( AttributeTypeContent {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedAttrType::Null,
                    "issuer".to_string() => NestedAttrType::Null,
                }),
            })),
        };

        let remove_command2 = Command {
            kind: CommandType::Remove,
            object_kind: ObjectKind::CaptureBase,
            content: Some(CommandContent::AttributeTypes( AttributeTypeContent {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedAttrType::Null,
                    "photo".to_string() => NestedAttrType::Null,
                }),
            })),
        };

        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        ocaast.commands.push(command2);
        let mut result = rule_remove_attr_if_exist(&ocaast, remove_command.clone());
        assert!(result.is_ok());
        ocaast.commands.push(remove_command2);
        result = rule_remove_attr_if_exist(&ocaast, remove_command);
        assert!(result.is_err());
    }

    #[test]
    fn test_rule_add_if_not_exist() {
        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase,
            content: Some( CommandContent::AttributeTypes( AttributeTypeContent {
                attributes: Some(indexmap! {
                    "documentType".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "photo".to_string() => NestedAttrType::Value(AttributeType::Binary),
                }),
            })),
        };

        let command2 = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase,
            content: Some( CommandContent::AttributeTypes( AttributeTypeContent {
                attributes: Some(indexmap! {
                    "issuer".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "last_name".to_string() => NestedAttrType::Value(AttributeType::Binary),
                }),
            })),
        };

        let add_command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase,
            content: Some( CommandContent::AttributeTypes( AttributeTypeContent {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "address".to_string() => NestedAttrType::Value(AttributeType::Text),
                }),
            })),
        };

        let add_command2 = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase,
            content: Some( CommandContent::AttributeTypes( AttributeTypeContent {
                attributes: Some(indexmap! {
                    "address".to_string() => NestedAttrType::Value(AttributeType::Text),
                    "phone".to_string() => NestedAttrType::Value(AttributeType::Text),
                }),
            })),
        };

        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        ocaast.commands.push(command2);
        let mut result = rule_add_attr_if_not_exist(&ocaast, add_command.clone());
        assert!(result.is_ok());
        ocaast.commands.push(add_command2);
        result = rule_add_attr_if_not_exist(&ocaast, add_command);
        assert!(result.is_err());
    }
}
