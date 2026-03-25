pub mod error;

use log::debug;
pub use oca_ast::ast::OCAAst;
mod instructions;

use self::{
    error::ParseError,
    instructions::{add::AddInstruction, from::FromInstruction, remove::RemoveInstruction},
};
use crate::ocafile::error::InstructionError;
use oca_ast::{
    ast::{
        self, Command, CommandMeta, NestedAttrType, RefValue,
        recursive_attributes::NestedAttrTypeFrame,
    },
    validator::{OCAValidator, Validator},
};
use overlay_file::overlay_registry::OverlayRegistry;
use pest::Parser;
use recursion::CollapsibleExt;

#[derive(pest_derive::Parser)]
#[grammar = "ocafile.pest"]
pub struct OCAfileParser;

pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub trait TryFromPair {
    type Error;
    fn try_from_pair(
        pair: Pair<'_>,
        registry: &dyn OverlayRegistry,
    ) -> Result<Command, Self::Error>;
}

impl TryFromPair for Command {
    type Error = InstructionError;
    fn try_from_pair(
        record: Pair,
        registry: &dyn OverlayRegistry,
    ) -> std::result::Result<Self, Self::Error> {
        let instruction: Command = match record.as_rule() {
            Rule::from => FromInstruction::from_record(record, 0)?,
            Rule::add => AddInstruction::from_record(record, 0, registry)?,
            Rule::remove => RemoveInstruction::from_record(record, 0)?,
            _ => return Err(InstructionError::UnexpectedToken(record.to_string())),
        };
        Ok(instruction)
    }
}

pub fn parse_from_string(
    unparsed_file: String,
    registry: &dyn OverlayRegistry,
) -> Result<OCAAst, ParseError> {
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
                        return Err(ParseError::MetaError(attr.as_str().to_string()));
                    }
                }
            }
            if key.is_empty() {
                return Err(ParseError::MetaError("key is empty".to_string()));
            }
            if value.is_empty() {
                return Err(ParseError::MetaError("value is empty".to_string()));
            }
            oca_ast.meta.insert(key, value);
            continue;
        }
        if let Rule::empty_line = line.as_rule() {
            continue;
        }

        match Command::try_from_pair(line.clone(), registry) {
            Ok(command) => match validator.validate(&oca_ast, command.clone()) {
                Ok(_) => {
                    oca_ast.commands.push(command);
                    oca_ast.commands_meta.insert(
                        oca_ast.commands.len() - 1,
                        CommandMeta {
                            line_number: n + 1,
                            raw_line: line.as_str().to_string().to_lowercase(),
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
                return Err(ParseError::InstructionError(e));
            }
        };
    }
    Ok(oca_ast)
}

// Format reference to oca file syntax
fn format_reference(ref_value: RefValue) -> String {
    match ref_value {
        RefValue::Said(said) => format!("refs:{}", said),
        _ => panic!("Unsupported reference type: {:?}", ref_value),
    }
}

// Convert NestedAttrType to oca file syntax
fn oca_file_format(nested: NestedAttrType) -> String {
    nested.collapse_frames(|frame| match frame {
        NestedAttrTypeFrame::Reference(ref_value) => format_reference(ref_value),
        NestedAttrTypeFrame::Value(value) => {
            format!("{}", value)
        }
        // TODO how to convert nested arrays?
        NestedAttrTypeFrame::Array(arr) => {
            format!("[{}]", arr)
        }
        NestedAttrTypeFrame::Null => "".to_string(),
    })
}

fn format_nested_value(value: &ast::NestedValue, indent: usize) -> String {
    match value {
        ast::NestedValue::Value(v) => v.to_string(),
        ast::NestedValue::Reference(ref_value) => format_reference(ref_value.clone()),
        ast::NestedValue::Object(obj) => obj
            .iter()
            .map(|(k, v)| {
                let formatted_value = format_nested_value(v, indent + 2);
                if v.is_object() {
                    format!("{}{}\n{}", " ".repeat(indent), k, formatted_value)
                } else if v.is_array() || v.is_reference() {
                    format!("{}{}={}", " ".repeat(indent), k, formatted_value)
                } else {
                    format!("{}{}=\"{}\"", " ".repeat(indent), k, formatted_value)
                }
            })
            .collect::<Vec<_>>()
            .join("\n"),
        ast::NestedValue::Array(arr) => {
            let formatted = arr
                .iter()
                .map(|v| format_nested_value(v, 0))
                .collect::<Vec<_>>();
            let formatted = formatted.join("\", \"");
            format!("[\"{}\"]", formatted)
        }
    }
}

/// Generate OCA file from AST
///
/// # Arguments
/// * `ast` - AST
///
pub fn generate_from_ast(ast: &OCAAst) -> String {
    let mut ocafile = String::new();

    ast.commands.iter().for_each(|command| {
        let mut line = String::new();

        debug!("Processing command: {:?}", command);
        match command.kind {
            ast::CommandType::Add => {
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
                    ast::ObjectKind::Overlay(content) => {
                        line.push_str("Overlay ");
                        let name = content.overlay_def.get_name();
                        line.push_str(name);
                        if let Some(content) = command.object_kind.overlay_content()
                            && let Some(ref properties) = content.properties
                        {
                            let properties = properties.clone();
                            if !properties.is_empty() {
                                line.push('\n');
                                properties.iter().for_each(|(key, value)| {
                                    let formatted_value = format_nested_value(value, 4);
                                    if value.is_object() {
                                        line.push_str(&format!("  {}\n{}\n", key, formatted_value));
                                    } else if value.is_array() {
                                        line.push_str(&format!("  {}={}\n", key, formatted_value));
                                    } else if value.is_reference() {
                                        line.push_str(&format!("  {}={}\n", key, value));
                                    } else {
                                        line.push_str(&format!(
                                            "  {}=\"{}\"\n",
                                            key, formatted_value
                                        ));
                                    }
                                });
                            }
                        };
                    }
                    _ => {
                        return;
                    }
                }
            }
            ast::CommandType::Remove => match &command.object_kind {
                ast::ObjectKind::CaptureBase(content) => {
                    line.push_str("REMOVE ");
                    if let Some(attributes) = &content.attributes {
                        line.push_str("ATTRIBUTE");
                        for (key, _) in attributes {
                            line.push_str(&format!(" {}", key));
                        }
                    }
                }
                ast::ObjectKind::Overlay(_) => {
                    todo!()
                }
                _ => {}
            },
            ast::CommandType::From => {
                line.push_str("FROM ");
            }
            ast::CommandType::Modify => todo!(),
        }

        ocafile.push_str(format!("{}\n", line).as_str());
    });

    ocafile
}

#[cfg(test)]
mod tests {
    use oca_ast::ast::AttributeType;
    use overlay_file::overlay_registry::OverlayLocalRegistry;
    use said::derivation::{HashFunction, HashFunctionCode};

    use super::{error::ExtractingAttributeError, *};

    #[test]
    fn parse_from_string_valid() {
        let _ = env_logger::builder().is_test(true).try_init();
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays").unwrap();

        let unparsed_file = r#"
-- version=2.0.0
-- name=プラスウルトラ
ADD ATTRIBUTE remove=Text
ADD ATTRIBUTE name=Text
    age=Numeric
    car=[refs:EJeWVGxkqxWrdGi0efOzwg1YQK8FrA-ZmtegiVEtAVcu]
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
        assert_eq!(oca_ast.meta.get("version").unwrap(), "2.0.0");
        assert_eq!(oca_ast.meta.get("name").unwrap(), "プラスウルトラ");
        assert_eq!(oca_ast.commands.len(), 15);
        let character_encoding_overlay = oca_ast.commands[6].object_kind.clone();
        assert_eq!(
            character_encoding_overlay
                .overlay_content()
                .unwrap()
                .overlay_def
                .get_full_name(),
            "character_encoding/2.0.0".to_string()
        );
    }

    #[test]
    fn parse_meta_from_string_valid() {
        let _ = env_logger::builder().is_test(true).try_init();
        let unparsed_file = r#"
-- version=0.0.1
-- name=Objekt
ADD attribute name=Text age=Numeric
"#;

        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays").unwrap();
        let oca_ast = parse_from_string(unparsed_file.to_string(), &registry).unwrap();
        assert_eq!(oca_ast.meta.get("version").unwrap(), "0.0.1");
        assert_eq!(oca_ast.meta.get("name").unwrap(), "Objekt");
    }

    #[test]
    fn test_deserialization_ast_to_ocafile() {
        let _ = env_logger::builder().is_test(true).try_init();
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays").unwrap();
        let unparsed_file = r#"ADD ATTRIBUTE name=Text age=Numeric radio=Text list=Text
ADD Overlay LABEL
  language="eo"
  attribute_labels
    name="Nomo"
    age="aĝo"
    radio="radio"

ADD Overlay CHARACTER_ENCODING
  attribute_character_encodings
    name="utf-8"
    age="utf-8"

ADD Overlay ENTRY_CODE
  attribute_entry_codes
    radio=["o1", "o2", "o3"]

ADD Overlay ENTRY
  language="eo"
  attribute_entries
    radio
      o1="etikedo1"
      o2="etikedo2"
      o3="etikiedo3"

ADD Overlay ENTRY
  language="pl"
  attribute_entries
    radio
      o1="etykieta1"
      o2="etykieta2"
      o3="etykieta3"

ADD Overlay ENTRY_CODE
  attribute_entry_codes
    list
      g1=["el1"]
      g2=["el2", "el3"]

ADD Overlay ENTRY
  language="pl"
  attribute_entries
    list
      el1="element1"
      el2="element2"
      el3="element3"
      g1="grupa1"
      g2="grupa2"

"#;
        let oca_ast = parse_from_string(unparsed_file.to_string(), &registry).unwrap();

        let ocafile = generate_from_ast(&oca_ast);
        let oca_ast2 = parse_from_string(ocafile.clone(), &registry).unwrap();
        assert_eq!(oca_ast, oca_ast2,);
    }

    #[test]
    fn test_attributes_with_special_names() {
        let _ = env_logger::builder().is_test(true).try_init();
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays").unwrap();
        let unparsed_file = r#"ADD ATTRIBUTE person.name=Text Experiment...Range..original.values.=[Text]
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string(), &registry).unwrap();

        let ocafile = generate_from_ast(&oca_ast);
        assert_eq!(
            ocafile, unparsed_file,
            "left:\n{} \n right:\n {}",
            ocafile, unparsed_file
        );
    }

    #[test]
    fn test_attributes_from_ast_to_ocafile() {
        let _ = env_logger::builder().is_test(true).try_init();
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays").unwrap();
        let unparsed_file = r#"ADD ATTRIBUTE name=Text age=Numeric
ADD ATTRIBUTE list=[Text] el=Text
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string(), &registry).unwrap();

        let ocafile = generate_from_ast(&oca_ast);
        assert_eq!(
            ocafile, unparsed_file,
            "left:\n{} \n right:\n {}",
            ocafile, unparsed_file
        );
    }

    #[test]
    fn test_nested_attributes_from_ocafile_to_ast() {
        let _ = env_logger::builder().is_test(true).try_init();
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays").unwrap();
        let unparsed_file = r#"ADD ATTRIBUTE name=Text age=Numeric car=[[Text]]
ADD ATTRIBUTE incidentals_spare_parts=[refs:EJVVlVSZJqVNnuAMLHLkeSQgwfxYLWTKBELi9e8j1PW0]
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string(), &registry).unwrap();

        let ocafile = generate_from_ast(&oca_ast);
        assert_eq!(
            ocafile, unparsed_file,
            "left:\n{} \n right:\n {}",
            ocafile, unparsed_file
        );
    }

    #[test]
    fn test_wrong_said() {
        let _ = env_logger::builder().is_test(true).try_init();
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays").unwrap();
        let unparsed_file = r#"ADD ATTRIBUTE said=refs:digest"#;
        let oca_ast = parse_from_string(unparsed_file.to_string(), &registry);
        match oca_ast.unwrap_err() {
            ParseError::InstructionError(InstructionError::ExtractError(
                ExtractingAttributeError::SaidError(e),
            )) => {
                assert_eq!(e.to_string(), "Unknown code")
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_oca_file_format() {
        let _ = env_logger::builder().is_test(true).try_init();
        let text_type = NestedAttrType::Value(AttributeType::Text);
        assert_eq!(oca_file_format(text_type), "Text");

        let numeric_type = NestedAttrType::Value(AttributeType::Numeric);
        assert_eq!(oca_file_format(numeric_type), "Numeric");

        let ref_type = NestedAttrType::Reference(RefValue::Said(
            HashFunction::from(HashFunctionCode::Blake3_256).derive("example".as_bytes()),
        ));

        let attr = NestedAttrType::Array(Box::new(NestedAttrType::Array(Box::new(ref_type))));

        let out = oca_file_format(attr);
        assert_eq!(out, "[[refs:EJeWVGxkqxWrdGi0efOzwg1YQK8FrA-ZmtegiVEtAVcu]]");
    }

    #[test]
    fn test_line_breaking_in_ocafile() {
        let _ = env_logger::builder().is_test(true).try_init();
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays").unwrap();

        // Test multi-line attribute definitions
        let unparsed_file = r#"
ADD ATTRIBUTE dateOfBirth=DateTime \
    documentNumber=Text
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string(), &registry).unwrap();

        // Verify that attributes are parsed correctly despite line breaks
        assert_eq!(oca_ast.commands.len(), 1);

        // Verify first command has all attributes
        if let ast::ObjectKind::CaptureBase(content) = &oca_ast.commands[0].object_kind {
            let attributes = content.attributes.as_ref().unwrap();
            assert_eq!(attributes.len(), 2);
            assert!(attributes.contains_key("dateOfBirth"));
            assert!(attributes.contains_key("documentNumber"));
        } else {
            panic!("Expected CaptureBase");
        }

        // Test that regenerated ocafile maintains structure
        let ocafile = generate_from_ast(&oca_ast);
        let oca_ast2 = parse_from_string(ocafile.clone(), &registry).unwrap();

        if let ast::ObjectKind::CaptureBase(content) = &oca_ast2.commands[0].object_kind {
            let attributes = content.attributes.as_ref().unwrap();
            assert_eq!(attributes.len(), 2);
            assert!(attributes.contains_key("dateOfBirth"));
            assert!(attributes.contains_key("documentNumber"));
        } else {
            panic!("Expected CaptureBase");
        }
    }

    #[test]
    fn test_attribute_type_case_insensitive() {
        let unparsed_file = r#"ADD ATTRIBUTE a=Text b=TEXT c=text d=TeXt e=Numeric f=NUMERIC g=numeric h=BoOlEaN i=BINARY j=datetime"#;

        let registry = OverlayLocalRegistry::new();
        let oca_ast = parse_from_string(unparsed_file.to_string(), &registry).unwrap();
        let attrs = oca_ast.commands[0]
            .object_kind
            .capture_content()
            .unwrap()
            .attributes
            .as_ref()
            .unwrap();
        assert_eq!(
            attrs.get("a").unwrap(),
            &NestedAttrType::Value(AttributeType::Text)
        );
        assert_eq!(
            attrs.get("b").unwrap(),
            &NestedAttrType::Value(AttributeType::Text)
        );
        assert_eq!(
            attrs.get("c").unwrap(),
            &NestedAttrType::Value(AttributeType::Text)
        );
        assert_eq!(
            attrs.get("d").unwrap(),
            &NestedAttrType::Value(AttributeType::Text)
        );
        assert_eq!(
            attrs.get("e").unwrap(),
            &NestedAttrType::Value(AttributeType::Numeric)
        );
        assert_eq!(
            attrs.get("f").unwrap(),
            &NestedAttrType::Value(AttributeType::Numeric)
        );
        assert_eq!(
            attrs.get("g").unwrap(),
            &NestedAttrType::Value(AttributeType::Numeric)
        );
        assert_eq!(
            attrs.get("h").unwrap(),
            &NestedAttrType::Value(AttributeType::Boolean)
        );
        assert_eq!(
            attrs.get("i").unwrap(),
            &NestedAttrType::Value(AttributeType::Binary)
        );
        assert_eq!(
            attrs.get("j").unwrap(),
            &NestedAttrType::Value(AttributeType::DateTime)
        );
    }

    #[test]
    fn test_language_variants_roundtrip() {
        let _ = env_logger::builder().is_test(true).try_init();
        let registry = OverlayLocalRegistry::from_dir("../overlay-file/core_overlays").unwrap();
        let language_cases = [
            "PL", "pl", "pol", "pl_PL", "en_UK", "eng", "EN", "en", "en-UK",
        ];

        for lang in language_cases {
            let unparsed_file = format!(
                "ADD ATTRIBUTE name=Text\nADD Overlay LABEL\n  language=\"{}\"\n  attribute_labels\n    name=\"Name\"\n",
                lang
            );

            let oca_ast = parse_from_string(unparsed_file, &registry).unwrap();
            let ocafile = generate_from_ast(&oca_ast);
            let oca_ast2 = parse_from_string(ocafile, &registry).unwrap();

            assert_eq!(oca_ast, oca_ast2);
        }
    }
}
