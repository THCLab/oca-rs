use indexmap::IndexMap;
use serde::{Serialize, Serializer};
use strum_macros::Display;


#[derive(Debug, PartialEq, Serialize)]
pub struct OCAAst {
    pub version: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Command {
    #[serde(rename = "type")]
    pub kind: CommandType,
    pub object_kind: ObjectKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
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

#[derive(Debug, PartialEq, Serialize)]
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

#[derive(Debug, PartialEq, Serialize, Display, Clone)]
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

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<IndexMap<String, NestedValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IndexMap<String, NestedValue>>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
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
