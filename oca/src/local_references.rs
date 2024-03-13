use std::{collections::HashMap, str::FromStr};

use oca_ast::ast::{CommandType, NestedAttrType, OCAAst, ObjectKind, RefValue};
use said::SelfAddressingIdentifier;

use crate::facade::build::Error;

// Iterate over all commands and dereference all attribute references
pub fn replace_refn_with_refs(oca_ast: &mut OCAAst, references: HashMap<String, String>) -> Result<(), Error> {
    for command in oca_ast.commands.iter_mut() {
        if let (CommandType::Add, ObjectKind::CaptureBase(content)) =
            (&command.kind, &mut command.object_kind)
        {
            if let Some(attributes) = &mut content.attributes {
                for (_, attr_type) in attributes {
                    match attr_type {
                        NestedAttrType::Reference(RefValue::Name(refn)) => {
                            if let Some(said) = references.get(refn) {
                                let said = SelfAddressingIdentifier::from_str(said).unwrap(); // todo
                                *attr_type = NestedAttrType::Reference(RefValue::Said(said));
                            } else {
                                return Err(Error::UnknownRefn(refn.clone()))
                            }
                        }
                        NestedAttrType::Array(box_attr_type) => {
                            if let NestedAttrType::Reference(RefValue::Name(refn)) =
                                &**box_attr_type
                            {
                                if let Some(said) = references.get(refn) {
                                    let said = SelfAddressingIdentifier::from_str(said).unwrap(); // todo
                                    **box_attr_type =
                                        NestedAttrType::Reference(RefValue::Said(said));
                                } else {
                                    return Err(Error::UnknownRefn(refn.clone()))
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    };
    Ok(())
}
