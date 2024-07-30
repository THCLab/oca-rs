use crate::{
    ast::{Command, NestedValue, TransformationAST},
    errors::Error,
};
use indexmap::{indexmap, IndexMap};

type CaptureAttributes = IndexMap<String, String>;

/// Validates given commands against existing valid OCA AST
///
/// # Arguments
/// * `ast` - valid OCA AST
/// * `command` - Command to validate against AST
///
/// # Returns
/// * `Result<bool, Error>` - Result of validation
pub trait Validator {
    fn validate(&self, ast: &TransformationAST, command: Command) -> Result<bool, Error>;
}

pub struct OCAValidator {}

impl Validator for OCAValidator {
    fn validate(&self, ast: &TransformationAST, command: Command) -> Result<bool, Error> {
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

fn validate_1_0_0(ast: &TransformationAST, command: Command) -> Result<bool, Error> {
    // Rules
    // Cannot remove if does not exist on stack
    // Cannot modify if does not exist on stack
    // Cannot add if already exists on stack
    // Attributes must have valid type
    let valid = true;
    let errors = Vec::new();
    match (&command.kind, &command.object_kind) {
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

fn extract_attributes(ast: &TransformationAST) -> CaptureAttributes {
    let default_attrs: IndexMap<String, String> = indexmap! {};
    let mut attributes: CaptureAttributes = indexmap! {};
    for instruction in &ast.commands {
        match (instruction.kind.clone(), instruction.object_kind.clone()) {
            _ => {}
        }
    }
    attributes
}

fn extract_properties(ast: &TransformationAST) -> IndexMap<String, NestedValue> {
    let default_attrs: IndexMap<String, NestedValue> = indexmap! {};
    let mut properties: IndexMap<String, NestedValue> = indexmap! {};
    for instruction in &ast.commands {
        match (instruction.kind.clone(), instruction.object_kind.clone()) {
            _ => {}
        }
    }
    properties
}

#[cfg(test)]
mod tests {
    /* #[test]
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

        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        let mut result = rule_remove_attr_if_exist(&ocaast, valid_command.clone());
        assert!(result.is_ok());
        ocaast.commands.push(invalid_command.clone());
        result = rule_remove_attr_if_exist(&ocaast, invalid_command);
        assert!(result.is_err());
    } */
}
