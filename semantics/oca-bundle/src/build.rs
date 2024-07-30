use crate::state::oca::overlay::cardinality::Cardinalitys;
use crate::state::oca::overlay::character_encoding::CharacterEncodings;
use crate::state::oca::overlay::conditional::Conditionals;
use crate::state::oca::overlay::conformance::Conformances;
use crate::state::oca::overlay::entry::Entries;
use crate::state::oca::overlay::entry_code::EntryCodes;
#[cfg(feature = "format_overlay")]
use crate::state::oca::overlay::format::Formats;
use crate::state::oca::overlay::information::Information;
use crate::state::oca::overlay::label::Labels;
use crate::state::oca::overlay::meta::Metas;
use crate::state::oca::overlay::unit::{
    AttributeUnit, ImperialUnit, MeasurementSystem, MeasurementUnit, MetricUnit, Unit,
};
use crate::state::oca::OCABundle;
use crate::state::{
    attribute::Attribute, encoding::Encoding, entries::EntriesElement,
    entry_codes::EntryCodes as EntryCodesValue, oca::OCABox,
};
use indexmap::IndexMap;
use oca_ast::ast;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct OCABuild {
    pub oca_bundle: OCABundle,
    pub steps: Vec<OCABuildStep>,
}

#[derive(Debug)]
pub struct OCABuildStep {
    pub parent_said: Option<said::SelfAddressingIdentifier>,
    pub command: ast::Command,
    pub result: OCABundle,
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

pub fn from_ast(
    from_oca: Option<OCABundle>,
    oca_ast: &ast::OCAAst,
) -> Result<OCABuild, Vec<Error>> {
    let mut errors = vec![];
    let mut steps = vec![];
    let mut parent_said: Option<said::SelfAddressingIdentifier> = match &from_oca {
        Some(oca_bundle) => oca_bundle.said.clone(),
        None => None,
    };
    let mut base: Option<OCABox> = from_oca.clone().map(OCABox::from);
    let default_command_meta = ast::CommandMeta {
        line_number: 0,
        raw_line: "unknown".to_string(),
    };
    for (i, command) in oca_ast.commands.iter().enumerate() {
        let command_index = match &from_oca {
            Some(_) => i + 1,
            None => i,
        };
        // todo pass the references
        let command_meta = oca_ast
            .commands_meta
            .get(&command_index)
            .unwrap_or(&default_command_meta);
        match apply_command(base.clone(), command.clone()) {
            Ok(oca_box) => {
                let mut oca_box_mut = oca_box.clone();
                let oca_bundle = oca_box_mut.generate_bundle();
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
                    parent_said.clone_from(&oca_bundle.said);
                    base = Some(oca_box);
                //}
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
        Ok(OCABuild {
            oca_bundle: base.unwrap().generate_bundle(),
            steps,
        })
    } else {
        Err(errors)
    }
}

pub fn apply_command(base: Option<OCABox>, op: ast::Command) -> Result<OCABox, Vec<String>> {
    let mut errors = vec![];
    let mut oca: OCABox = match base {
        Some(oca) => oca,
        None => OCABox::new(),
    };

    match (op.kind, op.object_kind) {
        (ast::CommandType::From, _) => {
            errors.push(
                "Unsupported FROM command, it should be resolved before applying commands"
                    .to_string(),
            );
        }
        (ast::CommandType::Add, ast::ObjectKind::CaptureBase(content)) => {
            if let Some(ref attributes) = content.attributes {
                for (attr_name, attr_type) in attributes {
                    let mut attribute = Attribute::new(attr_name.clone());
                    attribute.set_attribute_type(attr_type.clone());
                    oca.add_attribute(attribute);
                }
            }
            if let Some(ref properties) = content.properties {
                // TODO handle other properties
                for (prop_name, prop_value) in properties {
                    if prop_name.eq("classification") {
                        if let ast::NestedValue::Value(value) = prop_value {
                            oca.add_classification(value.clone());
                        }
                    }
                }
            }
        }
        (ast::CommandType::Add, ast::ObjectKind::Overlay(overlay_type, content)) => {
            match overlay_type {
                ast::OverlayType::Meta => {
                    if let Some(ref properties) = content.properties {
                        let mut mut_properties = properties.clone();
                        let lang = mut_properties.remove("lang");
                        let mut lang_iso = None;
                        if let Some(ast::NestedValue::Value(lang_str)) = lang {
                            lang_iso = isolang::Language::from_639_1(lang_str.as_str());
                        }

                        for (prop_name, prop_value) in mut_properties {
                            if let ast::NestedValue::Value(value) = prop_value {
                                oca.add_meta(lang_iso.unwrap(), prop_name.clone(), value.clone());
                            }
                        }
                    }
                }
                ast::OverlayType::Label => {
                    let mut lang_iso = None;
                    if let Some(ref properties) = content.properties {
                        let mut mut_properties = properties.clone();
                        let lang = mut_properties.remove("lang");
                        if let Some(ast::NestedValue::Value(lang_str)) = lang {
                            lang_iso = isolang::Language::from_639_1(lang_str.as_str());
                        }
                    }
                    if let Some(ref attributes) = content.attributes {
                        for (attr_name, attr_type_value) in attributes {
                            let mut attribute = oca
                                .attributes
                                .get(attr_name)
                                .ok_or_else(|| {
                                    errors.push(format!("Undefined attribute: {attr_name}"));
                                    errors.clone()
                                })?
                                .clone();
                            if let ast::NestedValue::Value(attr_label) = attr_type_value {
                                attribute.set_label(lang_iso.unwrap(), attr_label.clone());
                            }
                            oca.add_attribute(attribute);
                        }
                    }
                }
                ast::OverlayType::Information => {
                    let mut lang_iso = None;
                    if let Some(ref properties) = content.properties {
                        let mut mut_properties = properties.clone();
                        let lang = mut_properties.remove("lang");
                        if let Some(ast::NestedValue::Value(lang_str)) = lang {
                            lang_iso = isolang::Language::from_639_1(lang_str.as_str());
                        }
                    }
                    if let Some(ref attributes) = content.attributes {
                        for (attr_name, attr_type_value) in attributes {
                            let mut attribute = oca
                                .attributes
                                .get(attr_name)
                                .ok_or_else(|| {
                                    errors.push(format!("Undefined attribute: {attr_name}"));
                                    errors.clone()
                                })?
                                .clone();
                            if let ast::NestedValue::Value(attr_info) = attr_type_value {
                                attribute.set_information(lang_iso.unwrap(), attr_info.clone());
                            }
                            oca.add_attribute(attribute);
                        }
                    }
                }
                ast::OverlayType::CharacterEncoding => {
                    if let Some(ref attributes) = content.attributes {
                        for (attr_name, attr_type_value) in attributes {
                            let mut attribute = oca
                                .attributes
                                .get(attr_name)
                                .ok_or_else(|| {
                                    errors.push(format!("Undefined attribute: {attr_name}"));
                                    errors.clone()
                                })?
                                .clone();
                            if let ast::NestedValue::Value(attr_encoding) = attr_type_value {
                                attribute.set_encoding(Encoding::from_str(attr_encoding).map_err(
                                    |_| {
                                        errors.push(format!("Unknown encoding: {attr_encoding}"));
                                        errors.clone()
                                    },
                                )?);
                            }
                            oca.add_attribute(attribute);
                        }
                    }
                }
                ast::OverlayType::Conformance => {
                    if let Some(ref attributes) = content.attributes {
                        for (attr_name, attr_type_value) in attributes {
                            let mut attribute = oca
                                .attributes
                                .get(attr_name)
                                .ok_or_else(|| {
                                    errors.push(format!("Undefined attribute: {attr_name}"));
                                    errors.clone()
                                })?
                                .clone();

                            if let ast::NestedValue::Value(attr_conformance) = attr_type_value {
                                attribute.set_conformance(attr_conformance.clone());
                            }
                            oca.add_attribute(attribute);
                        }
                    }
                }
                ast::OverlayType::Format => {
                    #[cfg(feature = "format_overlay")]
                    {
                        if let Some(ref attributes) = content.attributes {
                            for (attr_name, attr_type_value) in attributes {
                                let mut attribute = oca
                                    .attributes
                                    .get(attr_name)
                                    .ok_or_else(|| {
                                        errors.push(format!("Undefined attribute: {attr_name}"));
                                        errors.clone()
                                    })?
                                    .clone();
                                if let ast::NestedValue::Value(attr_format) = attr_type_value {
                                    attribute.set_format(attr_format.clone());
                                }
                                oca.add_attribute(attribute);
                            }
                        }
                    }
                }
                ast::OverlayType::Unit => {
                    let mut unit_system_op = None;
                    if let Some(ref properties) = content.properties {
                        let mut mut_properties = properties.clone();
                        let unit_system_prop = mut_properties.remove("unit_system");
                        if let Some(ast::NestedValue::Value(unit_system_str)) = unit_system_prop {
                            unit_system_op = MeasurementSystem::from_str(&unit_system_str).ok();
                        }
                    }
                    if let Some(unit_system) = unit_system_op {
                        if let Some(ref attributes) = content.attributes {
                            for (attr_name, attr_type_value) in attributes {
                                let mut attribute = oca
                                    .attributes
                                    .get(attr_name)
                                    .ok_or_else(|| {
                                        errors.push(format!("Undefined attribute: {attr_name}"));
                                        errors.clone()
                                    })?
                                    .clone();
                                if let ast::NestedValue::Value(attr_unit) = attr_type_value {
                                    let unit = match unit_system {
                                        MeasurementSystem::Metric => Some(MeasurementUnit::Metric(
                                            MetricUnit::from_str(attr_unit).unwrap_or_else(|_| {
                                                panic!("{}", "Invalid metric unit: {attr_unit}")
                                            }),
                                        )),
                                        MeasurementSystem::Imperial => {
                                            Some(MeasurementUnit::Imperial(
                                                ImperialUnit::from_str(attr_unit).unwrap_or_else(
                                                    |_| {
                                                        panic!(
                                                            "{}",
                                                            "Invalid imperial unit: {attr_unit}"
                                                        )
                                                    },
                                                ),
                                            ))
                                        }
                                    };
                                    attribute.set_unit(AttributeUnit {
                                        measurement_system: unit_system.clone(),
                                        unit: unit.unwrap(),
                                    });
                                }
                                oca.add_attribute(attribute);
                            }
                        }
                    }
                }
                ast::OverlayType::Cardinality => {
                    if let Some(ref attributes) = content.attributes {
                        for (attr_name, attr_type_value) in attributes {
                            let mut attribute = oca
                                .attributes
                                .get(attr_name)
                                .ok_or_else(|| {
                                    errors.push(format!("Undefined attribute: {attr_name}"));
                                    errors.clone()
                                })?
                                .clone();
                            if let ast::NestedValue::Value(attr_cardinality) = attr_type_value {
                                attribute.set_cardinality(attr_cardinality.clone());
                            }
                            oca.add_attribute(attribute);
                        }
                    }
                }
                ast::OverlayType::Conditional => {
                    if let Some(ref attributes) = content.attributes {
                        for (attr_name, attr_type_value) in attributes {
                            let mut attribute = oca
                                .attributes
                                .get(attr_name)
                                .ok_or_else(|| {
                                    errors.push(format!("Undefined attribute: {attr_name}"));
                                    errors.clone()
                                })?
                                .clone();
                            if let ast::NestedValue::Value(attr_condition) = attr_type_value {
                                attribute.set_condition(attr_condition.clone());
                            }
                            oca.add_attribute(attribute);
                        }
                    }
                }
                ast::OverlayType::EntryCode => {
                    if let Some(ref attributes) = content.attributes {
                        for (attr_name, attr_type_value) in attributes {
                            let mut attribute = oca
                                .attributes
                                .get(attr_name)
                                .ok_or_else(|| {
                                    errors.push(format!("Undefined attribute: {attr_name}"));
                                    errors.clone()
                                })?
                                .clone();
                            match attr_type_value {
                                ast::NestedValue::Value(attr_entry_codes_sai) => {
                                    attribute.set_entry_codes(EntryCodesValue::Sai(
                                        attr_entry_codes_sai.clone(),
                                    ));
                                }
                                ast::NestedValue::Array(attr_entry_codes) => {
                                    let mut entry_codes: Vec<String> = vec![];
                                    for attr_entry_code in attr_entry_codes {
                                        if let ast::NestedValue::Value(entry_code) = attr_entry_code
                                        {
                                            entry_codes.push(entry_code.clone());
                                        }
                                    }
                                    attribute.set_entry_codes(EntryCodesValue::Array(entry_codes));
                                }
                                ast::NestedValue::Object(attr_grouped_entry_codes) => {
                                    let mut grouped_entry_codes = IndexMap::new();
                                    for (group, attr_entry_codes) in attr_grouped_entry_codes {
                                        if let ast::NestedValue::Array(entry_codes) = attr_entry_codes
                                        {
                                            let codes: Vec<String> = entry_codes
                                                .iter()
                                                .filter_map(|entry_code| {
                                                    if let ast::NestedValue::Value(entry_code) =
                                                        entry_code
                                                    {
                                                        Some(entry_code.clone())
                                                    } else {
                                                        None
                                                    }
                                                })
                                                .collect();
                                            grouped_entry_codes.insert(group.clone(), codes.clone());
                                        }
                                    }
                                    attribute.set_entry_codes(EntryCodesValue::Object(grouped_entry_codes));
                                }
                                _ => (),
                            }
                            oca.add_attribute(attribute);
                        }
                    }
                }
                ast::OverlayType::Entry => {
                    let mut lang_iso = None;
                    if let Some(ref properties) = content.properties {
                        let mut mut_properties = properties.clone();
                        let lang = mut_properties.remove("lang");
                        if let Some(ast::NestedValue::Value(lang_str)) = lang {
                            lang_iso = isolang::Language::from_639_1(lang_str.as_str());
                        }
                    }
                    if let Some(ref attributes) = content.attributes {
                        for (attr_name, attr_type_value) in attributes {
                            let mut attribute = oca
                                .attributes
                                .get(attr_name)
                                .ok_or_else(|| {
                                    errors.push(format!("Undefined attribute: {attr_name}"));
                                    errors.clone()
                                })?
                                .clone();
                            match attr_type_value {
                                ast::NestedValue::Value(attr_entries) => {
                                    attribute.set_entry(
                                        lang_iso.unwrap(),
                                        EntriesElement::Sai(attr_entries.clone()),
                                    );
                                }
                                ast::NestedValue::Object(attr_entries) => {
                                    let mut entries = HashMap::new();
                                    for (attr_entry_key, attr_entry_value) in attr_entries {
                                        if let ast::NestedValue::Value(entry_value) =
                                            attr_entry_value
                                        {
                                            entries.insert(
                                                attr_entry_key.clone(),
                                                entry_value.clone(),
                                            );
                                        }
                                    }
                                    attribute.set_entry(
                                        lang_iso.unwrap(),
                                        EntriesElement::Object(entries),
                                    );
                                }
                                _ => (),
                            }
                            oca.add_attribute(attribute);
                        }
                    }
                }
                _ => (),
            }
        }
        (ast::CommandType::Add, ast::ObjectKind::OCABundle(_)) => todo!(),
        (ast::CommandType::Remove, ast::ObjectKind::CaptureBase(content)) => {
            if let Some(ref attributes) = content.attributes {
                for (attr_name, _) in attributes {
                    oca.remove_attribute(attr_name);
                }
            }
            if let Some(ref properties) = content.properties {
                for (prop_name, _) in properties {
                    if prop_name.eq("classification") {
                        oca.remove_classification()
                    }
                }
            }
        }
        (ast::CommandType::Remove, ast::ObjectKind::OCABundle(_)) => todo!(),
        (ast::CommandType::Remove, ast::ObjectKind::Overlay(_, _)) => todo!(),
        (ast::CommandType::Modify, ast::ObjectKind::CaptureBase(_)) => todo!(),
        (ast::CommandType::Modify, ast::ObjectKind::OCABundle(_)) => todo!(),
        (ast::CommandType::Modify, ast::ObjectKind::Overlay(_, _)) => todo!(),
    }

    if errors.is_empty() {
        Ok(oca)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use oca_ast::ast::{AttributeType, CaptureContent};
    use said::version::Encode;

    #[test]
    fn test_add_step() {
        let mut commands = vec![];

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
                ast::Content {
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
                ast::Content {
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
                ast::Content {
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
                ast::Content {
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
                ast::Content {
                    attributes: Some(attributes),
                    properties: None,
                },
            ),
        });

        // todo test if references with name are working
        let mut base: Option<OCABox> = None;
        for command in commands {
            match apply_command(base.clone(), command.clone()) {
                Ok(oca) => {
                    base = Some(oca);
                }
                Err(errors) => {
                    println!("{:?}", errors);
                }
            }
            // let mut oca = apply_command(base, command);
            // println!("{}", serde_json::to_string_pretty(&oca.generate_bundle()).unwrap());
            // base = Some(oca);
        }
    }

    #[test]
    fn build_from_ast() {
        let mut commands = vec![];

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
            "list".to_string(),
            ast::NestedAttrType::Value(AttributeType::Text),
        );
        attributes.insert(
            "passed".to_string(),
            ast::NestedAttrType::Value(AttributeType::Boolean),
        );

        let flagged_attributes = vec!["d".to_string(), "i".to_string()];
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::CaptureBase(ast::CaptureContent {
                attributes: Some(attributes),
                properties: None,
                flagged_attributes: Some(flagged_attributes.clone()),
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
                ast::Content {
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
                ast::Content {
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
                ast::Content {
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
                ast::Content {
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
                ast::Content {
                    attributes: Some(attributes),
                    properties: None,
                },
            ),
        });

        let mut attributes = IndexMap::new();
        let mut grouped_elements = IndexMap::new();
        grouped_elements.insert("g1".to_string(), ast::NestedValue::Array(
            vec![ast::NestedValue::Value("el1".to_string())]
        ));
        grouped_elements.insert("g2".to_string(), ast::NestedValue::Array(
            vec![ast::NestedValue::Value("el2".to_string()), ast::NestedValue::Value("el3".to_string())]
        ));
        attributes.insert("list".to_string(), oca_ast::ast::NestedValue::Object(grouped_elements));
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(
                ast::OverlayType::EntryCode,
                ast::Content {
                    attributes: Some(attributes),
                    properties: None,
                },
            ),
        });

        let oca_ast = ast::OCAAst {
            version: "1.0".to_string(),
            commands,
            commands_meta: IndexMap::new(),
            meta: HashMap::new(),
        };

        let build_result = from_ast(None, &oca_ast);
        match build_result {
            Ok(oca_build) => {
                let oca_bundle_encoded = oca_build.oca_bundle.encode().unwrap();
                let oca_bundle_json = String::from_utf8(oca_bundle_encoded).unwrap();
                println!("{}", oca_bundle_json);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
