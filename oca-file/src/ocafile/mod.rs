mod error;
mod instructions;

use std::{collections::HashMap, str::FromStr};

use self::instructions::{add::AddInstruction, from::FromInstruction, remove::RemoveInstruction};
use crate::ocafile::error::Error;
use convert_case::{Case, Casing};
use oca_ast::{
    ast::{self, Command, CommandMeta, OCAAst, RefValue, NestedAttrType, attributes::NestedAttrTypeFrame},
    validator::{OCAValidator, Validator},
};
use pest::Parser;
use said::SelfAddressingIdentifier;
use recursion::CollapsibleExt;

#[derive(pest_derive::Parser)]
#[grammar = "ocafile.pest"]
pub struct OCAfileParser;

pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub trait TryFromPair {
    type Error;
    fn try_from_pair(pair: Pair<'_>) -> Result<Command, Self::Error>;
}

impl TryFromPair for Command {
    type Error = Error;
    fn try_from_pair(record: Pair) -> std::result::Result<Self, Self::Error> {
        let instruction: Command = match record.as_rule() {
            Rule::from => FromInstruction::from_record(record, 0)?,
            Rule::add => AddInstruction::from_record(record, 0)?,
            Rule::remove => RemoveInstruction::from_record(record, 0)?,
            _ => return Err(Error::UnexpectedToken(record.to_string())),
        };
        Ok(instruction)
    }
}

#[derive(thiserror::Error, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum ParseError {
    #[error("Error at line {line_number} ({raw_line}): {message}")]
    GrammarError {
        #[serde(rename = "ln")]
        line_number: usize,
        #[serde(rename = "col")]
        column_number: usize,
        #[serde(rename = "c")]
        raw_line: String,
        #[serde(rename = "e")]
        message: String,
    },
    #[error("{0}")]
    Custom(String),
}

pub fn parse_from_string(unparsed_file: String) -> Result<OCAAst, ParseError> {
    let file = OCAfileParser::parse(Rule::file, &unparsed_file)
        .map_err(|e| {
            let (line_number, column_number) = match e.line_col {
                pest::error::LineColLocation::Pos((line, column)) => (line, column),
                pest::error::LineColLocation::Span((line, column), _) => (line, column),
            };
            ParseError::GrammarError {
                line_number,
                column_number,
                raw_line: e.line().to_string(),
                message: e.variant.to_string(),
            }
        })?
        .next()
        .unwrap();

    let mut oca_ast = OCAAst::new();

    let validator = OCAValidator {};

    for (n, line) in file.into_inner().enumerate() {
        if let Rule::EOI = line.as_rule() {
            continue;
        }
        if let Rule::comment = line.as_rule() {
            continue;
        }
        if let Rule::meta_comment = line.as_rule() {
            let mut key = "".to_string();
            let mut value = "".to_string();
            for attr in line.into_inner() {
                match attr.as_rule() {
                    Rule::meta_attr_key => {
                        key = attr.as_str().to_string();
                    }
                    Rule::meta_attr_value => {
                        value = attr.as_str().to_string();
                    }
                    _ => {
                        return Err(ParseError::Custom(format!(
                            "Error parsing meta: {}",
                            attr.as_str()
                        )));
                    }
                }
            }
            if key == "" {
                return Err(ParseError::Custom(format!(
                    "Error parsing meta: key is empty"
                )));
            }
            if value == "" {
                return Err(ParseError::Custom(format!(
                    "Error parsing meta: value is empty"
                )));
            }
            oca_ast.meta.insert(key, value);
            continue;
        }
        if let Rule::empty_line = line.as_rule() {
            continue;
        }

        match Command::try_from_pair(line.clone()) {
            Ok(command) => match validator.validate(&oca_ast, command.clone()) {
                Ok(_) => {
                    oca_ast.commands.push(command);
                    oca_ast.commands_meta.insert(
                        oca_ast.commands.len() - 1,
                        CommandMeta {
                            line_number: n + 1,
                            raw_line: line.as_str().to_string(),
                        },
                    );
                }
                Err(e) => {
                    return Err(ParseError::Custom(format!(
                        "Error validating instruction: {}",
                        e
                    )));
                }
            },
            Err(e) => {
                return Err(ParseError::Custom(format!(
                    "Error parsing instruction: {}",
                    e
                )));
            }
        };
    }
    Ok(oca_ast)
}

// Format reference to oca file syntax
fn format_reference(ref_value: RefValue) -> String {
    match ref_value {
        RefValue::Said(said) => format!("refs:{}", said),
        _ => panic!( "Unsupported reference type: {:?}", ref_value)
    }
}

// Convert NestedAttrType to oca file syntax
fn oca_file_format(nested: NestedAttrType) -> String {
    nested.collapse_frames(|frame| match frame {
        NestedAttrTypeFrame::Reference(ref_value) => format_reference(ref_value),
        NestedAttrTypeFrame::Value(value) => {
            format!("{}", value)
        }
        NestedAttrTypeFrame::Object(obj) => {
            let start = "Object({".to_string();
            let end = "})".to_string();
            let inner_data = obj
                .into_iter()
                .map(|(obj_key, obj_value)| format!("{}={}", obj_key, obj_value));
            let out = inner_data.collect::<Vec<_>>().join(", ");
            vec![start, out, end].join("")
        }
        NestedAttrTypeFrame::Array(arr) => {
            format!("Array[{}]", arr)
        }
        NestedAttrTypeFrame::Null => "".to_string(),
    })
}

/// Generate OCA file from AST
///
/// # Arguments
/// * `ast` - AST
/// * `references` - Optional references names and thier saids for dereferencing
///
/// If references are present, ast would be trevers and all refn would be replaced with refs

pub fn generate_from_ast(ast: &OCAAst) -> String {
    let mut ocafile = String::new();

    ast.commands.iter().for_each(|command| {
        let mut line = String::new();

        if let ast::CommandType::Add = command.kind {
            line.push_str("ADD ");
            match &command.object_kind {
                ast::ObjectKind::CaptureBase(content) => {
                    if let Some(attributes) = &content.attributes {
                        line.push_str("ATTRIBUTE");
                        for (key, value) in attributes {
                            line.push_str(&format!(" {}=", key));
                            // TODO avoid clone
                            let out = oca_file_format(value.clone());
                            line.push_str(&out);
                        }
                    }
                }
                ast::ObjectKind::Overlay(o_type, _) => match o_type {
                    ast::OverlayType::Meta => {
                        line.push_str("META ");
                        if let Some(content) = command.object_kind.overlay_content() {
                            if let Some(ref properties) = content.properties {
                                let mut properties = properties.clone();
                                if let Some(ast::NestedValue::Value(lang)) =
                                    properties.remove("lang")
                                {
                                    line.push_str(format!("{} ", lang).as_str());
                                }
                                if !properties.is_empty() {
                                    line.push_str("PROPS ");
                                    properties.iter().for_each(|(key, value)| {
                                        if let ast::NestedValue::Value(value) = value {
                                            line.push_str(
                                                format!(" {}=\"{}\"", key, value).as_str(),
                                            );
                                        }
                                    });
                                }
                            }
                        };
                    }
                    ast::OverlayType::Unit => {
                        line.push_str("UNIT ");
                        if let Some(content) = command.object_kind.overlay_content() {
                            if let Some(ref properties) = content.properties {
                                let mut properties = properties.clone();
                                if let Some(ast::NestedValue::Value(unit_system)) =
                                    properties.remove("unit_system")
                                {
                                    line.push_str(format!("{} ", unit_system).as_str());
                                }
                                if !properties.is_empty() {
                                    line.push_str("PROPS ");
                                    properties.iter().for_each(|(key, value)| {
                                        if let ast::NestedValue::Value(value) = value {
                                            line.push_str(
                                                format!(" {}=\"{}\"", key, value).as_str(),
                                            );
                                        }
                                    });
                                }
                                if let Some(ref attributes) = content.attributes {
                                    line.push_str("ATTRS");
                                    attributes.iter().for_each(|(key, value)| {
                                        if let ast::NestedValue::Value(value) = value {
                                            line.push_str(
                                                format!(" {}=\"{}\"", key, value).as_str(),
                                            );
                                        }
                                    });
                                }
                            }
                        };
                    }
                    ast::OverlayType::EntryCode => {
                        line.push_str("ENTRY_CODE ");
                        if let Some(content) = command.object_kind.overlay_content() {
                            if let Some(ref properties) = content.properties {
                                if !properties.is_empty() {
                                    line.push_str("PROPS ");
                                    properties.iter().for_each(|(key, value)| {
                                        if let ast::NestedValue::Value(value) = value {
                                            line.push_str(format!(" {}={}", key, value).as_str());
                                        }
                                    });
                                }
                            }
                            if let Some(ref attributes) = content.attributes {
                                line.push_str("ATTRS");
                                attributes.iter().for_each(|(key, value)| {
                                    if let ast::NestedValue::Array(values) = value {
                                        let codes = values
                                            .iter()
                                            .filter_map(|value| {
                                                if let ast::NestedValue::Value(value) = value {
                                                    Some(format!("\"{}\"", value))
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect::<Vec<String>>()
                                            .join(", ");
                                        line.push_str(format!(" {}=[{}]", key, codes).as_str());
                                    } else if let ast::NestedValue::Value(said) = value {
                                        line.push_str(format!(" {}=\"{}\"", key, said).as_str());
                                    }
                                });
                            }
                        };
                    }
                    ast::OverlayType::Entry => {
                        line.push_str("ENTRY ");
                        if let Some(content) = command.object_kind.overlay_content() {
                            if let Some(ref properties) = content.properties {
                                let mut properties = properties.clone();
                                if let Some(ast::NestedValue::Value(lang)) =
                                    properties.remove("lang")
                                {
                                    line.push_str(format!("{} ", lang).as_str());
                                }
                                if !properties.is_empty() {
                                    line.push_str("PROPS ");
                                    properties.iter().for_each(|(key, value)| {
                                        if let ast::NestedValue::Value(value) = value {
                                            line.push_str(format!(" {}={}", key, value).as_str());
                                        }
                                    });
                                }
                                if let Some(ref attributes) = content.attributes {
                                    line.push_str("ATTRS ");
                                    attributes.iter().for_each(|(key, value)| {
                                        // TODO there is no need for NestedValue here
                                        if let ast::NestedValue::Object(values) = value {
                                            let codes = values
                                                .iter()
                                                .filter_map(|(code, label)| {
                                                    // TODO there is no need for NestedValue here
                                                    if let ast::NestedValue::Value(label) = label {
                                                        Some(format!("\"{}\": \"{}\"", code, label))
                                                    } else {
                                                        None
                                                    }
                                                })
                                                .collect::<Vec<String>>()
                                                .join(", ");
                                            line.push_str(
                                                format!("{}={{{}}}", key, codes).as_str(),
                                            );
                                        } else if let ast::NestedValue::Value(value) = value {
                                            line.push_str(
                                                format!(" {}=\"{}\"", key, value).as_str(),
                                            );
                                        }
                                    });
                                }
                            }
                        };
                    }
                    _ => {
                        line.push_str(
                            format!("{} ", o_type.to_string().to_case(Case::UpperSnake)).as_str(),
                        );

                        if let Some(content) = command.object_kind.overlay_content() {
                            if let Some(ref properties) = content.properties {
                                let mut properties = properties.clone();
                                if let Some(ast::NestedValue::Value(lang)) =
                                    properties.remove("lang")
                                {
                                    line.push_str(format!("{} ", lang).as_str());
                                }
                                if !properties.is_empty() {
                                    line.push_str("PROPS ");
                                    properties.iter().for_each(|(key, value)| {
                                        if let ast::NestedValue::Value(value) = value {
                                            line.push_str(
                                                format!(" {}=\"{}\"", key, value).as_str(),
                                            );
                                        }
                                    });
                                }
                            }
                            if let Some(ref attributes) = content.attributes {
                                line.push_str("ATTRS");
                                attributes.iter().for_each(|(key, value)| {
                                    if let ast::NestedValue::Value(value) = value {
                                        line.push_str(format!(" {}=\"{}\"", key, value).as_str());
                                    }
                                });
                            }
                        };
                    }
                },
                _ => {}
            }
        }

        ocafile.push_str(format!("{}\n", line).as_str());
    });

    ocafile
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use oca_ast::ast::AttributeType;
    use said::derivation::{HashFunction, HashFunctionCode};

    use super::*;

    #[test]
    fn parse_from_string_valid() {

        let _ = env_logger::builder().is_test(true).try_init();

        let unparsed_file = r#"
-- version=0.0.1
-- name=プラスウルトラ
ADD ATTRIBUTE name=Text age=Numeric car=Object({vin=Text, model=Text, year=Numeric})
ADD ATTRIBUTE incidentals_spare_parts=Array[Object({part_number=Text, description=Text, unit=Text, quantity=Numeric})]
ADD ATTRIBUTE d=Text i=Text passed=Boolean
ADD META en PROPS description="Entrance credential" name="Entrance credential"
ADD CHARACTER_ENCODING ATTRS d="utf-8" i="utf-8" passed="utf-8"
ADD CONFORMANCE ATTRS d="M" i="M" passed="M"
ADD LABEL en ATTRS d="Schema digest" i="Credential Issuee" passed="Passed"
ADD INFORMATION en ATTRS d="Name" i="Credential Issuee" passed="Enables or disables passing"
ADD FORMAT ATTRS d="image/jpeg"
ADD UNIT si ATTRS i=m
ADD ATTRIBUTE list=Array[Text] el=Text
ADD CARDINALITY ATTRS list="1-2"
ADD ENTRY_CODE ATTRS list="entry_code_said" el=["o1", "o2", "o3"]
ADD ENTRY en ATTRS list="entry_said" el={"o1": "o1_label", "o2": "o2_label", "o3": "o3_label"}
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string()).unwrap();
        assert_eq!(oca_ast.meta.get("version").unwrap(), "0.0.1");
        assert_eq!(oca_ast.meta.get("name").unwrap(), "プラスウルトラ");
    }

    #[test]
    fn parse_meta_from_string_valid() {
        let unparsed_file = r#"
-- version=0.0.1
-- name=Objekt
ADD attribute name=Text age=Numeric
"#;

        let oca_ast = parse_from_string(unparsed_file.to_string()).unwrap();
        assert_eq!(oca_ast.meta.get("version").unwrap(), "0.0.1");
        assert_eq!(oca_ast.meta.get("name").unwrap(), "Objekt");
    }

    #[test]
    fn test_deserialization_ast_to_ocafile() {
        let unparsed_file = r#"ADD ATTRIBUTE name=Text age=Numeric radio=Text
ADD LABEL eo ATTRS name="Nomo" age="aĝo" radio="radio"
ADD INFORMATION en ATTRS name="Object" age="Object"
ADD CHARACTER_ENCODING ATTRS name="utf-8" age="utf-8"
ADD ENTRY_CODE ATTRS radio=["o1", "o2", "o3"]
ADD ENTRY eo ATTRS radio={"o1": "etikedo1", "o2": "etikedo2", "o3": "etikiedo3"}
ADD ENTRY pl ATTRS radio={"o1": "etykieta1", "o2": "etykieta2", "o3": "etykieta3"}
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string()).unwrap();

        let ocafile = generate_from_ast(&oca_ast);
        assert_eq!(
            ocafile, unparsed_file,
            "left:\n{} \n right:\n {}",
            ocafile, unparsed_file
        );
    }

    #[test]
    fn test_attributes_from_ast_to_ocafile() {
        let unparsed_file = r#"ADD ATTRIBUTE name=Text age=Numeric
ADD ATTRIBUTE list=Array[Text] el=Text
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string()).unwrap();

        let ocafile = generate_from_ast(&oca_ast);
        assert_eq!(
            ocafile, unparsed_file,
            "left:\n{} \n right:\n {}",
            ocafile, unparsed_file
        );
    }

    #[test]
    fn test_nested_attributes_from_ocafile_to_ast() {
        let unparsed_file =
r#"ADD ATTRIBUTE name=Text age=Numeric car=Object({vin=Text, model=Text, year=Numeric})
ADD ATTRIBUTE incidentals_spare_parts=Array[Object({part_number=Text, description=Text, unit=Text, quantity=Numeric})]
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string()).unwrap();

        let ocafile = generate_from_ast(&oca_ast);
        assert_eq!(
            ocafile, unparsed_file,
            "left:\n{} \n right:\n {}",
            ocafile, unparsed_file
        );

    }


    #[test]
    fn test_oca_file_format() {
        let mut object_example = IndexMap::new();
        object_example.insert(
            "name".to_string(),
            NestedAttrType::Value(AttributeType::Text),
        );
        object_example.insert(
            "age".to_string(),
            NestedAttrType::Value(AttributeType::Numeric),
        );
        object_example.insert(
            "data".to_string(),
            NestedAttrType::Reference(RefValue::Said(
                HashFunction::from(HashFunctionCode::Blake3_256).derive("example".as_bytes()),
            )),
        );

        let attr = NestedAttrType::Array(Box::new(NestedAttrType::Object(object_example)));

        let out = oca_file_format(attr);
        assert_eq!(out, "Array[Object({name=Text, age=Numeric, data=refs:EJeWVGxkqxWrdGi0efOzwg1YQK8FrA-ZmtegiVEtAVcu})]");
    }
}
