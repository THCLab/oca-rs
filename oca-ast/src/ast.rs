use indexmap::IndexMap;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use strum_macros::Display;
use std::str::FromStr;
use wasm_bindgen::prelude::*;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OCAAst {
    pub version: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Command {
    #[serde(rename = "type")]
    pub kind: CommandType,
    pub object_kind: ObjectKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum CommandType {
    Add,
    Remove,
    Modify,
    From,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectKind {
    CaptureBase,
    OCABundle,
    Overlay(OverlayType),
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum AttributeType {
    Boolean,
    #[serde(rename = "Array[Boolean]")]
    ArrayBoolean,
    Binary,
    #[serde(rename = "Array[Binary]")]
    ArrayBinary,
    Text,
    #[serde(rename = "Array[Text]")]
    ArrayText,
    Numeric,
    #[serde(rename = "Array[Numeric]")]
    ArrayNumeric,
    DateTime,
    #[serde(rename = "Array[DateTime]")]
    ArrayDateTime,
    Reference,
    #[serde(rename = "Array[Reference]")]
    ArrayReference,
}

impl FromStr for AttributeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Boolean" => Ok(AttributeType::Boolean),
            "Array[Boolean]" => Ok(AttributeType::ArrayBoolean),
            "Binary" => Ok(AttributeType::Binary),
            "Array[Binary]" => Ok(AttributeType::ArrayBinary),
            "Text" => Ok(AttributeType::Text),
            "Array[Text]" => Ok(AttributeType::ArrayText),
            "Numeric" => Ok(AttributeType::Numeric),
            "Array[Numeric]" => Ok(AttributeType::ArrayNumeric),
            "DateTime" => Ok(AttributeType::DateTime),
            "Array[DateTime]" => Ok(AttributeType::ArrayDateTime),
            "Reference" => Ok(AttributeType::Reference),
            "Array[Reference]" => Ok(AttributeType::ArrayReference),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Display, Clone)]
pub enum OverlayType {
    Label,
    Information,
    Encoding,
    CharacterEncoding,
    Format,
    Meta,
    Standard,
    Cardinality,
    Conditional,
    Conformance,
    EntryCode,
    Entry,
    Unit,
    AttributeMapping,
    EntryCodeMapping,
    Subset,
    UnitMapping,
    Layout,
    Sensitivity,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<IndexMap<String, NestedValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IndexMap<String, NestedValue>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum NestedValue {
    Value(String),
    Object(IndexMap<String, NestedValue>),
    Reference(String),
    Array(Vec<NestedValue>),
}

impl OCAAst {
    pub fn new() -> Self {
        OCAAst {
            // Version of OCA specification
            version: String::from("1.0.0"),
            commands: Vec::new(),
        }
    }
}

impl Default for OCAAst {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for ObjectKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ObjectKind::CaptureBase => serializer.serialize_str("CaptureBase"),
            ObjectKind::OCABundle => serializer.serialize_str("OCABundle"),
            ObjectKind::Overlay(overlay_type) => {
                serializer.serialize_str(overlay_type.to_string().as_str())
            }
        }
    }
}

impl<'de> Deserialize<'de> for ObjectKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {

        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "CaptureBase" => Ok(ObjectKind::CaptureBase),
            "OCABundle" => Ok(ObjectKind::OCABundle),
            "Label" => Ok(ObjectKind::Overlay(OverlayType::Label)),
            "Information" => Ok(ObjectKind::Overlay(OverlayType::Information)),
            "Encoding" => Ok(ObjectKind::Overlay(OverlayType::Encoding)),
            "CharacterEncoding" => Ok(ObjectKind::Overlay(OverlayType::CharacterEncoding)),
            "Format" => Ok(ObjectKind::Overlay(OverlayType::Format)),
            "Meta" => Ok(ObjectKind::Overlay(OverlayType::Meta)),
            "Standard" => Ok(ObjectKind::Overlay(OverlayType::Standard)),
            "Cardinality" => Ok(ObjectKind::Overlay(OverlayType::Cardinality)),
            "Conditional" => Ok(ObjectKind::Overlay(OverlayType::Conditional)),
            "Conformance" => Ok(ObjectKind::Overlay(OverlayType::Conformance)),
            "EntryCode" => Ok(ObjectKind::Overlay(OverlayType::EntryCode)),
            "Entry" => Ok(ObjectKind::Overlay(OverlayType::Entry)),
            "Unit" => Ok(ObjectKind::Overlay(OverlayType::Unit)),
            "AttributeMapping" => Ok(ObjectKind::Overlay(OverlayType::AttributeMapping)),
            "EntryCodeMapping" => Ok(ObjectKind::Overlay(OverlayType::EntryCodeMapping)),
            "Subset" => Ok(ObjectKind::Overlay(OverlayType::Subset)),
            "UnitMapping" => Ok(ObjectKind::Overlay(OverlayType::UnitMapping)),
            "Layout" => Ok(ObjectKind::Overlay(OverlayType::Layout)),
            "Sensitivity" => Ok(ObjectKind::Overlay(OverlayType::Sensitivity)),
            _ => Err(serde::de::Error::custom(format!("unknown object kind: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocaast_serialize() {
        let mut attributes = IndexMap::new();
        let mut properties = IndexMap::new();
        let mut person = IndexMap::new();
        person.insert("name".to_string(), NestedValue::Value("Text".to_string()));

        attributes.insert("test".to_string(), NestedValue::Value("test".to_string()));
        attributes.insert("person".to_string(), NestedValue::Object(person));
        properties.insert("test".to_string(), NestedValue::Value("test".to_string()));
        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase,
            content: Some(Content {
                attributes: Some(attributes),
                properties: Some(properties),
            }),
        };

        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        let serialized = serde_json::to_string(&ocaast).unwrap();
        assert_eq!(
            serialized,
            r#"{"version":"1.0.0","commands":[{"type":"Add","object_kind":"CaptureBase","content":{"attributes":{"test":"test","person":{"name":"Text"}},"properties":{"test":"test"}}}]}"#
        );
    }
}
