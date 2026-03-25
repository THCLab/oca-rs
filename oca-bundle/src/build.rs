use crate::state::oca_bundle::OCABundleModel;
use crate::state::oca_bundle::overlay::OverlayModel;
use log::info;
use oca_ast::ast;
use oca_ast::ast::NestedValue;
use overlay_file::OverlayDef;

/// OCABuild represents a build process of an OCA bundle from OCA AST.
/// It contains the final OCA bundle and a list of steps that were applied to create it.
#[derive(Debug)]
pub struct OCABuild {
    pub oca_bundle: OCABundleModel,
    pub steps: Vec<OCABuildStep>,
}

#[derive(Debug)]
pub struct OCABuildStep {
    pub parent_said: Option<said::SelfAddressingIdentifier>,
    pub command: ast::Command,
    pub result: OCABundleModel,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FromASTError {
    pub line_number: usize,
    pub raw_line: String,
    pub message: String,
}

#[derive(thiserror::Error, Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Error {
    #[error("Error at line {line_number} ({raw_line}): {message}")]
    FromASTError {
        #[serde(rename = "ln")]
        line_number: usize,
        #[serde(rename = "c")]
        raw_line: String,
        #[serde(rename = "e")]
        message: String,
    },
}

/// Create a new OCA build from OCA AST
pub fn from_ast(
    from_bundle: Option<OCABundleModel>,
    oca_ast: &ast::OCAAst,
) -> Result<OCABuild, Vec<Error>> {
    let mut errors = vec![];
    let mut steps = vec![];
    let mut parent_said: Option<said::SelfAddressingIdentifier> = match &from_bundle {
        Some(oca_bundle) => oca_bundle.digest.clone(),
        None => None,
    };
    let has_from_bundle = from_bundle.is_some();

    let mut oca_bundle = from_bundle.unwrap_or_default();

    let default_command_meta = ast::CommandMeta {
        line_number: 0,
        raw_line: "unknown".to_string(),
    };
    for (i, command) in oca_ast.commands.iter().enumerate() {
        let command_index = if has_from_bundle { i + 1 } else { i };

        // todo pass the references
        let command_meta = oca_ast
            .commands_meta
            .get(&command_index)
            .unwrap_or(&default_command_meta);

        match apply_command(&mut oca_bundle, command.clone()) {
            Ok(oca_bundle) => {
                /* if oca_bundle.said == parent_said {
                    errors.push(Error::FromASTError {
                        line_number: command_meta.line_number,
                        raw_line: command_meta.raw_line.clone(),
                        message: "Applying command failed".to_string(),
                    });
                } else { */
                steps.push(OCABuildStep {
                    parent_said: parent_said.clone(),
                    command: command.clone(),
                    result: oca_bundle.clone(),
                });
                parent_said.clone_from(&oca_bundle.digest);
            }
            Err(mut err) => {
                errors.extend(err.iter_mut().map(|e| Error::FromASTError {
                    line_number: command_meta.line_number,
                    raw_line: command_meta.raw_line.clone(),
                    message: e.clone(),
                }));
            }
        }
    }
    if errors.is_empty() {
        Ok(OCABuild { oca_bundle, steps })
    } else {
        Err(errors)
    }
}

pub fn apply_command(
    base: &mut OCABundleModel,
    op: ast::Command,
) -> Result<&OCABundleModel, Vec<String>> {
    let mut errors = vec![];

    match (op.kind, op.object_kind) {
        (ast::CommandType::From, _) => {
            errors.push(
                "Unsupported FROM command, it should be resolved before applying commands"
                    .to_string(),
            );
        }
        (ast::CommandType::Add, ast::ObjectKind::CaptureBase(content)) => {
            if let Some(ref attributes) = content.attributes {
                base.capture_base.attributes.extend(attributes.clone());
            }
        }
        (ast::CommandType::Add, ast::ObjectKind::Overlay(content)) => {
            let mut overlay = OverlayModel::new(content.clone());
            overlay.overlay_def = Some(content.overlay_def);
            if let Err(err) = merge_or_add_overlay(base, overlay) {
                errors.push(err);
            }
        }
        (ast::CommandType::Add, ast::ObjectKind::OCABundle(_)) => todo!(),
        (ast::CommandType::Remove, ast::ObjectKind::CaptureBase(content)) => {
            if let Some(ref attributes) = content.attributes {
                for (attr_name, _) in attributes {
                    base.remove_attribute(attr_name);
                }
            }
        }
        (ast::CommandType::Remove, ast::ObjectKind::OCABundle(_)) => todo!(),
        (ast::CommandType::Remove, ast::ObjectKind::Overlay(_)) => todo!(),
        (ast::CommandType::Modify, ast::ObjectKind::CaptureBase(_)) => todo!(),
        (ast::CommandType::Modify, ast::ObjectKind::OCABundle(_)) => todo!(),
        (ast::CommandType::Modify, ast::ObjectKind::Overlay(_)) => todo!(),
    }
    // Calculate and fill digest for bundle, capture base and overlays
    match base.compute_and_fill_digest() {
        Ok(_) => info!("Digests filled successfully"),
        Err(e) => return Err(vec![format!("Error filling digests: {}", e)]),
    }
    base.fill_attributes();
    if errors.is_empty() {
        Ok(base)
    } else {
        Err(errors)
    }
}

fn merge_or_add_overlay(base: &mut OCABundleModel, incoming: OverlayModel) -> Result<(), String> {
    let overlay_def = incoming
        .overlay_def
        .as_ref()
        .ok_or_else(|| "Overlay definition missing".to_string())?;
    if overlay_def.unique_keys.is_empty() {
        base.overlays.push(incoming);
        return Ok(());
    }

    let incoming_properties = incoming.properties.as_ref().ok_or_else(|| {
        format!(
            "Overlay {} is missing properties for unique keys",
            overlay_def.get_full_name()
        )
    })?;

    let signature = unique_keys_signature(overlay_def, incoming_properties)?;
    let incoming_name = overlay_def.get_full_name();

    for existing in &mut base.overlays {
        let existing_def = match &existing.overlay_def {
            Some(def) => def,
            None => continue,
        };
        if existing_def.get_full_name() != incoming_name {
            continue;
        }
        let existing_props = match existing.properties.as_mut() {
            Some(props) => props,
            None => {
                return Err(format!(
                    "Overlay {} is missing properties for unique keys",
                    existing_def.get_full_name()
                ));
            }
        };
        let existing_signature = unique_keys_signature(existing_def, existing_props)?;
        if existing_signature != signature {
            continue;
        }

        // Merge properties for same overlay + unique signature.
        merge_properties(existing_props, incoming_properties)?;
        return Ok(());
    }

    base.overlays.push(incoming);
    Ok(())
}

fn unique_keys_signature(
    overlay_def: &OverlayDef,
    properties: &indexmap::IndexMap<String, NestedValue>,
) -> Result<String, String> {
    let mut parts = Vec::new();
    let mut missing = Vec::new();
    for key in &overlay_def.unique_keys {
        match properties.get(key) {
            Some(value) => {
                let value_str = serde_json::to_string(value).unwrap_or_else(|_| value.to_string());
                parts.push(format!("{}={}", key, value_str));
            }
            None => missing.push(key.clone()),
        }
    }
    if !missing.is_empty() {
        return Err(format!(
            "Overlay {} is missing unique keys: {}",
            overlay_def.get_full_name(),
            missing.join(", ")
        ));
    }
    Ok(parts.join("|"))
}

fn merge_properties(
    target: &mut indexmap::IndexMap<String, NestedValue>,
    incoming: &indexmap::IndexMap<String, NestedValue>,
) -> Result<(), String> {
    for (key, value) in incoming {
        match target.get_mut(key) {
            Some(existing) => merge_nested_value(existing, value, key)?,
            None => {
                target.insert(key.clone(), value.clone());
            }
        }
    }
    Ok(())
}

fn merge_nested_value(
    target: &mut NestedValue,
    incoming: &NestedValue,
    path: &str,
) -> Result<(), String> {
    match target {
        NestedValue::Object(target_obj) => match incoming {
            NestedValue::Object(incoming_obj) => {
                for (key, value) in incoming_obj.iter() {
                    if let Some(existing) = target_obj.get_mut(key) {
                        let nested_path = format!("{path}.{key}");
                        merge_nested_value(existing, value, &nested_path)?;
                    } else {
                        target_obj.insert(key.clone(), value.clone());
                    }
                }
                Ok(())
            }
            _ => {
                if target == incoming {
                    Ok(())
                } else {
                    Err(format!(
                        "Overlay attribute override for {path}: existing value differs"
                    ))
                }
            }
        },
        _ => {
            if target == incoming {
                Ok(())
            } else {
                Err(format!(
                    "Overlay attribute override for {path}: existing value differs"
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error;

    use crate::state::oca_bundle::OCABundle;

    use super::*;
    use indexmap::IndexMap;
    use oca_ast::ast::{AttributeType, CaptureContent};
    use oca_file::ocafile::parse_from_string;
    use overlay_file::overlay_registry::{OverlayLocalRegistry, OverlayRegistry};

    #[test]
    fn test_add_step() -> Result<(), Box<dyn std::error::Error>> {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut commands = vec![];

        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays/")?;

        let mut attributes = IndexMap::new();
        attributes.insert(
            "d".to_string(),
            ast::NestedAttrType::Value(AttributeType::Text),
        );
        attributes.insert(
            "i".to_string(),
            ast::NestedAttrType::Value(AttributeType::Text),
        );
        attributes.insert(
            "passed".to_string(),
            ast::NestedAttrType::Value(AttributeType::Boolean),
        );
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(attributes),
            }),
        });

        let mut properties = IndexMap::new();
        properties.insert(
            "language".to_string(),
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
        let meta_ov_def = registry.get_overlay("Meta/2.0.0").unwrap();
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(ast::OverlayContent {
                properties: Some(properties),
                overlay_def: meta_ov_def.clone(),
            }),
        });

        let mut properties = IndexMap::new();
        properties.insert(
            "d".to_string(),
            ast::NestedValue::Value("Schema digest".to_string()),
        );
        properties.insert(
            "i".to_string(),
            ast::NestedValue::Value("Credential Issuee".to_string()),
        );
        properties.insert(
            "passed".to_string(),
            ast::NestedValue::Value("Passed".to_string()),
        );
        let mut attr_labels = IndexMap::new();
        attr_labels.insert(
            "language".to_string(),
            ast::NestedValue::Value("en".to_string()),
        );
        attr_labels.insert(
            "attribute_labels".to_string(),
            ast::NestedValue::Object(properties.clone()),
        );
        let label_ov_def = registry.get_overlay("Label/2.0.0").unwrap();
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(ast::OverlayContent {
                properties: Some(attr_labels),
                overlay_def: label_ov_def.clone(),
            }),
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
            "language".to_string(),
            ast::NestedValue::Value("en".to_string()),
        );

        let mut properties = IndexMap::new();
        properties.insert(
            "d".to_string(),
            ast::NestedValue::Value("utf-8".to_string()),
        );
        properties.insert(
            "i".to_string(),
            ast::NestedValue::Value("utf-8".to_string()),
        );
        properties.insert(
            "passed".to_string(),
            ast::NestedValue::Value("utf-8".to_string()),
        );
        let character_encoding_ov_def = registry.get_overlay("Character_Encoding/2.0.0").unwrap();
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(ast::OverlayContent {
                properties: Some(properties),
                overlay_def: character_encoding_ov_def.clone(),
            }),
        });

        let mut properties = IndexMap::new();
        properties.insert("d".to_string(), ast::NestedValue::Value("M".to_string()));
        properties.insert("i".to_string(), ast::NestedValue::Value("M".to_string()));
        properties.insert(
            "passed".to_string(),
            ast::NestedValue::Value("M".to_string()),
        );
        let conformance_ov_def = registry.get_overlay("Conformance/2.0.0").unwrap();
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(ast::OverlayContent {
                properties: Some(properties),
                overlay_def: conformance_ov_def.clone(),
            }),
        });

        // todo test if references with name are working
        let mut base = OCABundleModel::default();

        for command in commands {
            match apply_command(&mut base, command.clone()) {
                Ok(oca) => {
                    println!("Bundle: {}", serde_json::to_string_pretty(oca)?);
                }
                Err(errors) => {
                    println!("Error applying command: {:?}", errors);
                    return Err(Box::new(Error::other(format!("{:?}", errors))));
                }
            }
        }
        assert_eq!(
            base.attributes.unwrap().len(),
            3,
            "Expected 5 attributes in the base after applying commands"
        );
        // TODO check if overlays are created correctly
        // let mut oca = apply_command(base, command);
        // base = Some(oca);
        Ok(())
    }

    #[test]
    fn merge_label_overlays_conflict_should_error_and_keep_single_overlay() {
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays/").unwrap();
        let ocafile = r#"
ADD ATTRIBUTE first_name=Text last_name=Text

ADD OVERLAY LABEL
  language="en"
  attribute_labels
    first_name="First Name"
    last_name="Last Name"

ADD OVERLAY LABEL
  language="en"
  attribute_labels
    first_name="Name"
    last_name="Name"
"#;

        let oca_ast = parse_from_string(ocafile.to_string(), &registry).unwrap();
        let mut base = OCABundleModel::default();
        let mut errors = Vec::new();
        for command in oca_ast.commands {
            if let Err(mut err) = apply_command(&mut base, command) {
                errors.append(&mut err);
            }
        }

        assert!(!errors.is_empty());
        assert_eq!(base.overlays.len(), 1);
    }

    #[test]
    fn merge_label_overlays_disjoint_should_merge() {
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays/").unwrap();
        let ocafile = r#"
ADD ATTRIBUTE first_name=Text last_name=Text description=Text info=Text

ADD OVERLAY LABEL
  language="en"
  attribute_labels
    first_name="First Name"
    last_name="Last Name"

ADD OVERLAY LABEL
  language="en"
  attribute_labels
    description="Name"
    info="Name"
"#;

        let oca_ast = parse_from_string(ocafile.to_string(), &registry).unwrap();
        let mut base = OCABundleModel::default();
        let mut errors = Vec::new();
        for command in oca_ast.commands {
            if let Err(mut err) = apply_command(&mut base, command) {
                errors.append(&mut err);
            }
        }

        assert!(errors.is_empty());
        assert_eq!(base.overlays.len(), 1);

        let properties = base.overlays[0].properties.as_ref().unwrap();
        let labels = properties.get("attribute_labels").unwrap();
        match labels {
            NestedValue::Object(map) => {
                assert!(map.contains_key("first_name"));
                assert!(map.contains_key("last_name"));
                assert!(map.contains_key("description"));
                assert!(map.contains_key("info"));
            }
            _ => panic!("attribute_labels should be an object"),
        }
    }

    #[test]
    fn build_from_ast() {
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays").unwrap();

        let unparsed_file = r#"
-- version=2.0.0
-- name=プラスウルトラ
ADD ATTRIBUTE remove=Text
ADD ATTRIBUTE name=Text age=Numeric car=[refs:EJeWVGxkqxWrdGi0efOzwg1YQK8FrA-ZmtegiVEtAVcu]
REMOVE ATTRIBUTE remove
ADD ATTRIBUTE incidentals_spare_parts=[[refs:EJeWVGxkqxWrdGi0efOzwg1YQK8FrA-ZmtegiVEtAVcu]]
ADD ATTRIBUTE d=Text i=Text passed=Boolean
ADD Overlay META
  language="en"
  description="Entrance credential"
  name="Entrance credential"
ADD Overlay CHARACTER_ENCODING
  attribute_character_encodings
    d="utf-8"
    i="utf-8"
    passed="utf-8"
ADD Overlay CONFORMANCE
  attribute_conformances=["d", "i", "passed"]
ADD Overlay LABEL
  language="en"
  attribute_labels
    d="Schema digest"
    i="Credential Issuee"
    passed="Passed"
ADD Overlay FORMAT
  attribute_formats
    d="image/jpeg"
ADD Overlay UNIT
  metric_system="SI"
  attribute_units
    i="m^2"
    d="°"
ADD ATTRIBUTE list=[Text] el=Text
ADD Overlay CARDINALITY
  attribute_cardinalities
    list="1-2"
ADD Overlay ENTRY_CODE
  attribute_entry_codes
    list=refs:EJeWVGxkqxWrdGi0efOzwg1YQK8FrA-ZmtegiVEtAVcu
    el=["o1", "o2", "o3"]
ADD Overlay ENTRY
  language="en"
  attribute_entries
    list=refs:EJeWVGxkqxWrdGi0efOzwg1YQK8FrA-ZmtegiVEtAVcu
    el
     o1="o1_label"
     o2="o2_label"
     o3="o3_label"
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string(), &registry).unwrap();

        let oca_bundle = OCABundleModel::default();

        let mut oca_bundle_model = from_ast(Some(oca_bundle), &oca_ast).unwrap().oca_bundle;
        let _ = oca_bundle_model.compute_and_fill_digest();
        assert_eq!(oca_bundle_model.overlays.len(), 9);
        assert_eq!(oca_bundle_model.capture_base.attributes.len(), 9);
        assert!(oca_bundle_model.digest.is_some());
        let oca_bundle = OCABundle::from(oca_bundle_model);
        let overlay_name = oca_bundle.overlays.first().unwrap().model.name.clone();
        assert_eq!(overlay_name, "meta");
        // let json = serde_json::to_string_pretty(&oca_bundle).unwrap();
        // println!(" >>> Bundle: {}", json);
    }
}
