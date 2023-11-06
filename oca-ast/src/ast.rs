use indexmap::IndexMap;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use strum_macros::Display;
use std::str::FromStr;
use wasm_bindgen::prelude::*;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OCAAst {
    pub version: String,
    pub commands: Vec<Command>,
    pub commands_meta: IndexMap<usize, CommandMeta>,
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
pub struct CommandMeta {
    pub line_number: usize,
    pub raw_line: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum CommandType {
    Add,
    Remove,
    Modify,
    From,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Display, Clone)]
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

impl Serialize for OverlayType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OverlayType::Label => {
                serializer.serialize_str("spec/overlays/label/1.0")
            }
            OverlayType::Information => {
                serializer.serialize_str("spec/overlays/information/1.0")
            }
            OverlayType::Encoding => {
                serializer.serialize_str("spec/overlays/encoding/1.0")
            }
            OverlayType::CharacterEncoding => {
                serializer.serialize_str("spec/overlays/character_encoding/1.0")
            }
            OverlayType::Format => {
                serializer.serialize_str("spec/overlays/format/1.0")
            }
            OverlayType::Meta => {
                serializer.serialize_str("spec/overlays/meta/1.0")
            }
            OverlayType::Standard => {
                serializer.serialize_str("spec/overlays/standard/1.0")
            }
            OverlayType::Cardinality => {
                serializer.serialize_str("spec/overlays/cardinality/1.0")
            }
            OverlayType::Conditional => {
                serializer.serialize_str("spec/overlays/conditional/1.0")
            }
            OverlayType::Conformance => {
                serializer.serialize_str("spec/overlays/conformance/1.0")
            }
            OverlayType::EntryCode => {
                serializer.serialize_str("spec/overlays/entry_code/1.0")
            }
            OverlayType::Entry => {
                serializer.serialize_str("spec/overlays/entry/1.0")
            }
            OverlayType::Unit => {
                serializer.serialize_str("spec/overlays/unit/1.0")
            }
            OverlayType::AttributeMapping => {
                serializer.serialize_str("spec/overlays/mapping/1.0")
            }
            OverlayType::EntryCodeMapping => {
                serializer.serialize_str("spec/overlays/entry_code_mapping/1.0")
            }
            OverlayType::Subset => {
                serializer.serialize_str("spec/overlays/subset/1.0")
            }
            OverlayType::UnitMapping => {
                serializer.serialize_str("spec/overlays/unit_mapping/1.0")
            }
            OverlayType::Layout => {
                serializer.serialize_str("spec/overlays/layout/1.0")
            }
            OverlayType::Sensitivity => {
                serializer.serialize_str("spec/overlays/sensitivity/1.0")
            }
        }
    }
}

impl<'de> Deserialize<'de> for OverlayType {
    fn deserialize<D>(deserializer: D) -> Result<OverlayType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "spec/overlays/label/1.0" => Ok(OverlayType::Label),
            "spec/overlays/information/1.0" => Ok(OverlayType::Information),
            "spec/overlays/encoding/1.0" => Ok(OverlayType::Encoding),
            "spec/overlays/character_encoding/1.0" => {
                Ok(OverlayType::CharacterEncoding)
            }
            "spec/overlays/format/1.0" => Ok(OverlayType::Format),
            "spec/overlays/meta/1.0" => Ok(OverlayType::Meta),
            "spec/overlays/standard/1.0" => Ok(OverlayType::Standard),
            "spec/overlays/cardinality/1.0" => Ok(OverlayType::Cardinality),
            "spec/overlays/conditional/1.0" => Ok(OverlayType::Conditional),
            "spec/overlays/conformance/1.0" => Ok(OverlayType::Conformance),
            "spec/overlays/entry_code/1.0" => Ok(OverlayType::EntryCode),
            "spec/overlays/entry/1.0" => Ok(OverlayType::Entry),
            "spec/overlays/unit/1.0" => Ok(OverlayType::Unit),
            "spec/overlays/mapping/1.0" => Ok(OverlayType::AttributeMapping),
            "spec/overlays/entry_code_mapping/1.0" => {
                Ok(OverlayType::EntryCodeMapping)
            }
            "spec/overlays/subset/1.0" => Ok(OverlayType::Subset),
            "spec/overlays/unit_mapping/1.0" => Ok(OverlayType::UnitMapping),
            "spec/overlays/layout/1.0" => Ok(OverlayType::Layout),
            "spec/overlays/sensitivity/1.0" => Ok(OverlayType::Sensitivity),
            _ => Err(serde::de::Error::custom(format!(
                "unknown overlay type: {}",
                s
            ))),
        }
    }
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
    Reference(RefValue),
    Value(String),
    Object(IndexMap<String, NestedValue>),
    Array(Vec<NestedValue>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum RefValue {
    Said(String),
    Name(String),
}

impl Serialize for RefValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            RefValue::Said(said) => serializer.serialize_str(
                format!("_said_:{}", said).as_str()
            ),
            RefValue::Name(name) => serializer.serialize_str(
                format!("_name_:{}", name).as_str()
            ),
        }
    }
}

impl<'de> Deserialize<'de> for RefValue {
    fn deserialize<D>(deserializer: D) -> Result<RefValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let (tag, rest) = s.split_once(':').ok_or(
            serde::de::Error::custom(format!("invalid reference: {}", s))
        )?;
        match tag {
            "_said_" => Ok(RefValue::Said(rest.to_string())),
            "_name_" => Ok(RefValue::Name(rest.to_string())),
            _ => Err(serde::de::Error::custom(format!(
                "unknown reference type: {}",
                tag
            ))),
        }
    }
}

impl OCAAst {
    pub fn new() -> Self {
        OCAAst {
            // Version of OCA specification
            version: String::from("1.0.0"),
            commands: Vec::new(),
            commands_meta: IndexMap::new(),
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

impl From<u8> for ObjectKind {
    fn from(val: u8) -> Self {
        match val {
            0 => ObjectKind::CaptureBase,
            1 => ObjectKind::OCABundle,
            2 => ObjectKind::Overlay(OverlayType::Label),
            3 => ObjectKind::Overlay(OverlayType::Information),
            4 => ObjectKind::Overlay(OverlayType::Encoding),
            5 => ObjectKind::Overlay(OverlayType::CharacterEncoding),
            6 => ObjectKind::Overlay(OverlayType::Format),
            7 => ObjectKind::Overlay(OverlayType::Meta),
            8 => ObjectKind::Overlay(OverlayType::Standard),
            9 => ObjectKind::Overlay(OverlayType::Cardinality),
            10 => ObjectKind::Overlay(OverlayType::Conditional),
            11 => ObjectKind::Overlay(OverlayType::Conformance),
            12 => ObjectKind::Overlay(OverlayType::EntryCode),
            13 => ObjectKind::Overlay(OverlayType::Entry),
            14 => ObjectKind::Overlay(OverlayType::Unit),
            15 => ObjectKind::Overlay(OverlayType::AttributeMapping),
            16 => ObjectKind::Overlay(OverlayType::EntryCodeMapping),
            17 => ObjectKind::Overlay(OverlayType::Subset),
            18 => ObjectKind::Overlay(OverlayType::UnitMapping),
            19 => ObjectKind::Overlay(OverlayType::Layout),
            20 => ObjectKind::Overlay(OverlayType::Sensitivity),
            _ => panic!("Unknown object type"),
        }
    }
}

impl From<ObjectKind> for u8 {
    fn from(val: ObjectKind) -> Self {
        match val {
            ObjectKind::CaptureBase => 0,
            ObjectKind::OCABundle => 1,
            ObjectKind::Overlay(OverlayType::Label) => 2,
            ObjectKind::Overlay(OverlayType::Information) => 3,
            ObjectKind::Overlay(OverlayType::Encoding) => 4,
            ObjectKind::Overlay(OverlayType::CharacterEncoding) => 5,
            ObjectKind::Overlay(OverlayType::Format) => 6,
            ObjectKind::Overlay(OverlayType::Meta) => 7,
            ObjectKind::Overlay(OverlayType::Standard) => 8,
            ObjectKind::Overlay(OverlayType::Cardinality) => 9,
            ObjectKind::Overlay(OverlayType::Conditional) => 10,
            ObjectKind::Overlay(OverlayType::Conformance) => 11,
            ObjectKind::Overlay(OverlayType::EntryCode) => 12,
            ObjectKind::Overlay(OverlayType::Entry) => 13,
            ObjectKind::Overlay(OverlayType::Unit) => 14,
            ObjectKind::Overlay(OverlayType::AttributeMapping) => 15,
            ObjectKind::Overlay(OverlayType::EntryCodeMapping) => 16,
            ObjectKind::Overlay(OverlayType::Subset) => 17,
            ObjectKind::Overlay(OverlayType::UnitMapping) => 18,
            ObjectKind::Overlay(OverlayType::Layout) => 19,
            ObjectKind::Overlay(OverlayType::Sensitivity) => 20,
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
            r#"{"version":"1.0.0","commands":[{"type":"Add","object_kind":"CaptureBase","content":{"attributes":{"test":"test","person":{"name":"Text"}},"properties":{"test":"test"}}}],"commands_meta":{}}"#
        );
    }
}
