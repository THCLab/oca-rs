use crate::data_storage::DataStorage;
use oca_ast_semantics::ast::{self, RefValue};
use oca_bundle_semantics::state::oca::{OCABox, OCABundle};
use said::{derivation::HashFunctionCode, sad::SerializationFormats, version::Encode};

pub fn build_oca(
    db: Box<dyn DataStorage>,
    commands: Vec<ast::Command>,
) -> Result<OCABundle, String> {
    let mut base: Option<OCABox> = None;
    for command in commands {
        if let ast::CommandType::From = command.kind {
            let said = match command.clone().object_kind.oca_bundle_content() {
                Some(oca_bundle_content) => match oca_bundle_content.clone().said {
                    ast::ReferenceAttrType::Reference(refs) => match refs {
                        RefValue::Said(said) => said.to_string(),
                        RefValue::Name(_) => return Err("Not implemented".to_string()),
                    },
                },
                None => return Err("Missing bundle content".to_string()),
            };
            let oca_bundle_str = match db.get(&format!("oca.{}", said))? {
                Some(oca_bundle_str) => String::from_utf8(oca_bundle_str).unwrap(),
                None => return Err("OCA not found".to_string()),
            };
            let oca_b = serde_json::from_str::<OCABundle>(&oca_bundle_str).unwrap();
            base = Some(oca_b.into());
        } else {
            let result = oca_bundle_semantics::build::apply_command(base.clone(), command.clone());
            let mut oca_box: OCABox = OCABox::new();
            match result {
                Ok(oca) => {
                    oca_box = oca;
                }
                Err(errors) => {
                    println!("{:?}", errors);
                }
            };
            let oca_bundle = oca_box.generate_bundle();
            let command_str = serde_json::to_string(&command).unwrap();

            let mut input: Vec<u8> = vec![];
            match base {
                Some(ref mut base) => {
                    let base_said = base.generate_bundle().said.unwrap().to_string();
                    input.push(base_said.len().try_into().unwrap());
                    input.extend(base_said.as_bytes());
                }
                None => {
                    input.push(0);
                }
            }
            input.push(command_str.len().try_into().unwrap());
            input.extend(command_str.as_bytes());
            db.insert(
                &format!("oca.{}.operation", oca_bundle.clone().said.unwrap()),
                &input,
            )?;
            let code = HashFunctionCode::Blake3_256;
            let format = SerializationFormats::JSON;
            db.insert(
                &format!("oca.{}", oca_bundle.clone().said.unwrap()),
                &oca_bundle.encode(&code, &format).unwrap(),
            )?;

            base = Some(oca_box);
        }
    }

    Ok(base.unwrap().generate_bundle())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::data_storage::{DataStorage, SledDataStorage};
    use indexmap::IndexMap;
    use oca_ast_semantics::ast::{BundleContent, CaptureContent, Content};
    use said::SelfAddressingIdentifier;

    #[test]
    #[ignore]
    fn test_ocafile_build() {
        let mut commands = vec![];

        let mut attributes = IndexMap::new();
        attributes.insert(
            "d".to_string(),
            ast::NestedAttrType::Value(ast::AttributeType::Text),
        );
        attributes.insert(
            "i".to_string(),
            ast::NestedAttrType::Value(ast::AttributeType::Text),
        );
        attributes.insert(
            "passed".to_string(),
            ast::NestedAttrType::Value(ast::AttributeType::Boolean),
        );
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(attributes),
                properties: None,
                flagged_attributes: None,
            }),
        });

        let mut properties = IndexMap::new();
        properties.insert(
            "lang".to_string(),
            ast::NestedValue::Value("en".to_string()),
        );
        properties.insert(
            "name".to_string(),
            ast::NestedValue::Value("Entrance credential".to_string()),
        );
        properties.insert(
            "description".to_string(),
            ast::NestedValue::Value("Entrance credential".to_string()),
        );
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(
                ast::OverlayType::Meta,
                Content {
                    attributes: None,
                    properties: Some(properties),
                },
            ),
        });

        let mut attributes = IndexMap::new();
        attributes.insert(
            "d".to_string(),
            ast::NestedValue::Value("Schema digest".to_string()),
        );
        attributes.insert(
            "i".to_string(),
            ast::NestedValue::Value("Credential Issuee".to_string()),
        );
        attributes.insert(
            "passed".to_string(),
            ast::NestedValue::Value("Passed".to_string()),
        );
        let mut properties = IndexMap::new();
        properties.insert(
            "lang".to_string(),
            ast::NestedValue::Value("en".to_string()),
        );
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(
                ast::OverlayType::Label,
                Content {
                    attributes: Some(attributes),
                    properties: Some(properties),
                },
            ),
        });

        let mut attributes = IndexMap::new();
        attributes.insert(
            "d".to_string(),
            ast::NestedValue::Value("Schema digest".to_string()),
        );
        attributes.insert(
            "i".to_string(),
            ast::NestedValue::Value("Credential Issuee".to_string()),
        );
        attributes.insert(
            "passed".to_string(),
            ast::NestedValue::Value("Enables or disables passing".to_string()),
        );
        let mut properties = IndexMap::new();
        properties.insert(
            "lang".to_string(),
            ast::NestedValue::Value("en".to_string()),
        );
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(
                ast::OverlayType::Information,
                Content {
                    attributes: Some(attributes),
                    properties: Some(properties),
                },
            ),
        });

        let mut attributes = IndexMap::new();
        attributes.insert(
            "d".to_string(),
            ast::NestedValue::Value("utf-8".to_string()),
        );
        attributes.insert(
            "i".to_string(),
            ast::NestedValue::Value("utf-8".to_string()),
        );
        attributes.insert(
            "passed".to_string(),
            ast::NestedValue::Value("utf-8".to_string()),
        );
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(
                ast::OverlayType::CharacterEncoding,
                Content {
                    attributes: Some(attributes),
                    properties: None,
                },
            ),
        });

        let mut attributes = IndexMap::new();
        attributes.insert("d".to_string(), ast::NestedValue::Value("M".to_string()));
        attributes.insert("i".to_string(), ast::NestedValue::Value("M".to_string()));
        attributes.insert(
            "passed".to_string(),
            ast::NestedValue::Value("M".to_string()),
        );
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(
                ast::OverlayType::Conformance,
                Content {
                    attributes: Some(attributes),
                    properties: None,
                },
            ),
        });

        let db = SledDataStorage::open("db_test");
        let oca = build_oca(Box::new(db), commands);

        let db_read = SledDataStorage::open("db_test");
        let op = db_read
            .get(&format!("oca.{}.operation", oca.unwrap().said.unwrap()))
            .unwrap();
        println!("{:?}", String::from_utf8_lossy(&op.unwrap()));
        // println!("{}", serde_json::to_string_pretty(&oca.unwrap()).unwrap());

        // assert_eq!(digests, vec![44, 69, 73, 74, 71, 74, 109, 83, 95, 80, 57, 106, 119, 90, 68, 97, 109, 66, 54, 99, 84, 71, 57, 77, 111, 88, 75, 82, 117, 50, 49, 109, 121, 106, 88, 115, 77, 105, 55, 71, 89, 100, 100, 68, 121])
    }

    #[test]
    #[ignore]
    fn test_ocafile_build_from() {
        let mut commands = vec![];

        let said =
            SelfAddressingIdentifier::from_str("EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs")
                .unwrap();
        let reference = ast::ReferenceAttrType::Reference(RefValue::Said(said));
        commands.push(ast::Command {
            kind: ast::CommandType::From,
            object_kind: ast::ObjectKind::OCABundle(BundleContent { said: reference }),
        });

        let mut attributes = IndexMap::new();
        attributes.insert(
            "new".to_string(),
            ast::NestedAttrType::Value(ast::AttributeType::Text),
        );
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(attributes),
                properties: None,
                flagged_attributes: None,
            }),
        });

        let db = SledDataStorage::open("db_test");
        let oca = build_oca(Box::new(db), commands);

        let code = HashFunctionCode::Blake3_256;
        let format = SerializationFormats::JSON;
        println!(
            "{:?}",
            String::from_utf8(oca.clone().unwrap().encode(&code, &format).unwrap())
        );

        let db_read = SledDataStorage::open("db_test");
        let op = db_read
            .get(&format!("oca.{}.operation", oca.unwrap().said.unwrap()))
            .unwrap();
        println!("{:?}", String::from_utf8_lossy(&op.unwrap()));
        // println!("{}", serde_json::to_string_pretty(&oca.unwrap()).unwrap());

        // assert_eq!(digests, vec![44, 69, 73, 74, 71, 74, 109, 83, 95, 80, 57, 106, 119, 90, 68, 97, 109, 66, 54, 99, 84, 71, 57, 77, 111, 88, 75, 82, 117, 50, 49, 109, 121, 106, 88, 115, 77, 105, 55, 71, 89, 100, 100, 68, 121])
    }
}
