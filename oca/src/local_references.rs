use std::str::FromStr;

use oca_ast::ast::{CommandType, NestedAttrType, OCAAst, ObjectKind, RefValue};
use said::SelfAddressingIdentifier;

use crate::facade::build::ValidationError;

pub trait References {
    fn find(&self, refn: &str) -> Option<String>;
    fn save(&mut self, refn: &str, value: String);
}

// Iterate over all commands and dereference all attribute references
pub fn replace_refn_with_refs<R: References>(
    oca_ast: &mut OCAAst,
    references: &R,
) -> Result<(), ValidationError> {
    for command in oca_ast.commands.iter_mut() {
        if let (CommandType::Add, ObjectKind::CaptureBase(content)) =
            (&command.kind, &mut command.object_kind)
        {
            if let Some(attributes) = &mut content.attributes {
                for (_, attr_type) in attributes {
                    match attr_type {
                        NestedAttrType::Reference(RefValue::Name(refn)) => {
                            if let Some(said) = references.find(refn) {
                                let said = SelfAddressingIdentifier::from_str(&said).unwrap(); // todo
                                *attr_type = NestedAttrType::Reference(RefValue::Said(said));
                            } else {
                                return Err(ValidationError::UnknownRefn(refn.clone()));
                            }
                        }
                        NestedAttrType::Array(box_attr_type) => {
                            if let NestedAttrType::Reference(RefValue::Name(refn)) =
                                &**box_attr_type
                            {
                                if let Some(said) = references.find(refn) {
                                    let said = SelfAddressingIdentifier::from_str(&said).unwrap(); // todo
                                    **box_attr_type =
                                        NestedAttrType::Reference(RefValue::Said(said));
                                } else {
                                    return Err(ValidationError::UnknownRefn(refn.clone()));
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    Ok(())
}
