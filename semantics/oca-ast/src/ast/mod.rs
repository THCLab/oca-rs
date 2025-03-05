use indexmap::IndexMap;
use said::SelfAddressingIdentifier;
use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{hash::Hash, sync::OnceLock};
use std::{collections::HashMap, fmt, str::FromStr};
use strum_macros::Display;
use thiserror::Error;
use wasm_bindgen::prelude::*;

pub use self::attributes::NestedAttrType;

pub mod attributes;
pub mod error;
pub mod recursive_attributes;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OCAAst {
    pub version: String,
    pub commands: Vec<Command>,
    pub commands_meta: IndexMap<usize, CommandMeta>,
    pub meta: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Command {
    #[serde(rename = "type")]
    pub kind: CommandType,
    #[serde(flatten)]
    pub object_kind: ObjectKind,
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

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum ObjectKind {
    CaptureBase(CaptureContent),
    OCABundle(BundleContent),
    Overlay(OverlayType, Content),
}

impl Hash for ObjectKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ObjectKind::CaptureBase(content) => {
                content.hash(state);
            }
            ObjectKind::OCABundle(content) => {
                content.hash(state);
            }
            // TODO hash over content as well?
            ObjectKind::Overlay(overlay_type, _) => {
                overlay_type.hash(state);
            }
        }
    }
}

impl Hash for CaptureContent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(attributes) = &self.attributes {
            for (key, value) in attributes {
                key.hash(state);
                value.hash(state);
            }
        }
        if let Some(properties) = &self.properties {
            for (key, value) in properties {
                key.hash(state);
                value.hash(state);
            }
        }
    }
}

impl ObjectKind {
    pub fn capture_content(&self) -> Option<&CaptureContent> {
        match self {
            ObjectKind::CaptureBase(content) => Some(content),
            _ => None,
        }
    }

    pub fn overlay_content(&self) -> Option<&Content> {
        match self {
            ObjectKind::Overlay(_, content) => Some(content),
            _ => None,
        }
    }
    pub fn oca_bundle_content(&self) -> Option<&BundleContent> {
        match self {
            ObjectKind::OCABundle(content) => Some(content),
            _ => None,
        }
    }
}
#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy, Display, Eq, Hash)]
pub enum AttributeType {
    Boolean,
    Binary,
    Text,
    Numeric,
    DateTime,
}

impl FromStr for AttributeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Boolean" => Ok(AttributeType::Boolean),
            "Binary" => Ok(AttributeType::Binary),
            "Text" => Ok(AttributeType::Text),
            "Numeric" => Ok(AttributeType::Numeric),
            "DateTime" => Ok(AttributeType::DateTime),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    Link,
    AttributeFraming,
}

impl Serialize for OverlayType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OverlayType::Label => serializer.serialize_str("spec/overlays/label/1.1"),
            OverlayType::Information => serializer.serialize_str("spec/overlays/information/1.1"),
            OverlayType::Encoding => serializer.serialize_str("spec/overlays/encoding/1.1"),
            OverlayType::CharacterEncoding => {
                serializer.serialize_str("spec/overlays/character_encoding/1.1")
            }
            OverlayType::Format => serializer.serialize_str("spec/overlays/format/1.1"),
            OverlayType::Meta => serializer.serialize_str("spec/overlays/meta/1.1"),
            OverlayType::Standard => serializer.serialize_str("spec/overlays/standard/1.1"),
            OverlayType::Cardinality => serializer.serialize_str("spec/overlays/cardinality/1.1"),
            OverlayType::Conditional => serializer.serialize_str("spec/overlays/conditional/1.1"),
            OverlayType::Conformance => serializer.serialize_str("spec/overlays/conformance/1.1"),
            OverlayType::EntryCode => serializer.serialize_str("spec/overlays/entry_code/1.1"),
            OverlayType::Entry => serializer.serialize_str("spec/overlays/entry/1.1"),
            OverlayType::Unit => serializer.serialize_str("spec/overlays/unit/1.1"),
            OverlayType::AttributeMapping => serializer.serialize_str("spec/overlays/mapping/1.1"),
            OverlayType::EntryCodeMapping => {
                serializer.serialize_str("spec/overlays/entry_code_mapping/1.1")
            }
            OverlayType::Subset => serializer.serialize_str("spec/overlays/subset/1.1"),
            OverlayType::UnitMapping => serializer.serialize_str("spec/overlays/unit_mapping/1.1"),
            OverlayType::Layout => serializer.serialize_str("spec/overlays/layout/1.1"),
            OverlayType::Sensitivity => serializer.serialize_str("spec/overlays/sensitivity/1.1"),
            OverlayType::Link => serializer.serialize_str("spec/overlays/link/1.1"),
            OverlayType::AttributeFraming => { 
                serializer.serialize_str("spec/overlays/attribute_framing/1.1")
            },
        }
    }
}

impl Serialize for ObjectKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ObjectKind", 3)?;
        match self {
            ObjectKind::CaptureBase(content) => {
                state.serialize_field("object_kind", "CaptureBase")?;
                state.serialize_field("content", content)?;
            }
            ObjectKind::OCABundle(content) => {
                state.serialize_field("object_kind", "OCABundle")?;
                state.serialize_field("content", content)?;
            }
            ObjectKind::Overlay(overlay_type, content) => {
                // Convert OverlayType to a string representation
                let overlay_type_str = overlay_type.to_string();
                state.serialize_field("object_kind", &overlay_type_str)?;
                state.serialize_field("content", content)?;
            }
        }
        state.end()
    }
}

impl FromStr for OverlayType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Label" => Ok(OverlayType::Label),
            "Information" => Ok(OverlayType::Information),
            "Encoding" => Ok(OverlayType::Encoding),
            "CharacterEncoding" => Ok(OverlayType::CharacterEncoding),
            "Format" => Ok(OverlayType::Format),
            "Meta" => Ok(OverlayType::Meta),
            "Standard" => Ok(OverlayType::Standard),
            "Cardinality" => Ok(OverlayType::Cardinality),
            "Conditional" => Ok(OverlayType::Conditional),
            "Conformance" => Ok(OverlayType::Conformance),
            "EntryCode" => Ok(OverlayType::EntryCode),
            "Entry" => Ok(OverlayType::Entry),
            "Unit" => Ok(OverlayType::Unit),
            "Mapping" => Ok(OverlayType::AttributeMapping),
            "EntryCodeMapping" => Ok(OverlayType::EntryCodeMapping),
            "Subset" => Ok(OverlayType::Subset),
            "UnitMapping" => Ok(OverlayType::UnitMapping),
            "Layout" => Ok(OverlayType::Layout),
            "Sensitivity" => Ok(OverlayType::Sensitivity),
            "Link" => Ok(OverlayType::Link),
            "AttributeFraming" => Ok(OverlayType::AttributeFraming),
            _ => Err(()),
        }
    }
}

impl fmt::Display for OverlayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OverlayType::Label => write!(f, "Label"),
            OverlayType::Information => write!(f, "Information"),
            OverlayType::Encoding => write!(f, "Encoding"),
            OverlayType::CharacterEncoding => write!(f, "CharacterEncoding"),
            OverlayType::Format => write!(f, "Format"),
            OverlayType::Meta => write!(f, "Meta"),
            OverlayType::Standard => write!(f, "Standard"),
            OverlayType::Cardinality => write!(f, "Cardinality"),
            OverlayType::Conditional => write!(f, "Conditional"),
            OverlayType::Conformance => write!(f, "Conformance"),
            OverlayType::EntryCode => write!(f, "EntryCode"),
            OverlayType::Entry => write!(f, "Entry"),
            OverlayType::Unit => write!(f, "Unit"),
            OverlayType::AttributeMapping => write!(f, "AttributeMapping"),
            OverlayType::EntryCodeMapping => write!(f, "EntryCodeMapping"),
            OverlayType::Subset => write!(f, "Subset"),
            OverlayType::UnitMapping => write!(f, "UnitMapping"),
            OverlayType::Layout => write!(f, "Layout"),
            OverlayType::Sensitivity => write!(f, "Sensitivity"),
            OverlayType::Link => write!(f, "Link"),
            OverlayType::AttributeFraming => write!(f, "AttributeFraming"),
        }
    }
}

static OVERLAY_PATTERN: OnceLock<regex::Regex> = OnceLock::new();

impl<'de> Deserialize<'de> for OverlayType {
    fn deserialize<D>(deserializer: D) -> Result<OverlayType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let overlay_type = String::deserialize(deserializer)?;
        let pattern = OVERLAY_PATTERN.get_or_init(|| {
            regex::Regex::new(r"^spec/overlays/(\w+)/(\d+\.\d+)$").unwrap()
        });

        if let Some(captures) = pattern.captures(&overlay_type) {
            match captures.get(1).unwrap().as_str() {
                "label" => Ok(OverlayType::Label),
                "format" => Ok(OverlayType::Format),
                "information" => Ok(OverlayType::Information),
                "encoding" => Ok(OverlayType::Encoding),
                "character_encoding" => Ok(OverlayType::CharacterEncoding),
                "meta" => Ok(OverlayType::Meta),
                "standard" => Ok(OverlayType::Standard),
                "cardinality" => Ok(OverlayType::Cardinality),
                "conditional" => Ok(OverlayType::Conditional),
                "conformance" => Ok(OverlayType::Conformance),
                "entry_code" => Ok(OverlayType::EntryCode),
                "entry" => Ok(OverlayType::Entry),
                "unit" => Ok(OverlayType::Unit),
                "mapping" => Ok(OverlayType::AttributeMapping),
                "entry_code_mapping" => Ok(OverlayType::EntryCodeMapping),
                "subset" => Ok(OverlayType::Subset),
                "unit_mapping" => Ok(OverlayType::UnitMapping),
                "layout" => Ok(OverlayType::Layout),
                "sensitivity" => Ok(OverlayType::Sensitivity),
                "link" => Ok(OverlayType::Link),
                "attribute_framing" => Ok(OverlayType::AttributeFraming),
                _ => Err(serde::de::Error::custom("Unknown overlay type"))
            }
        } else {
            Err(serde::de::Error::custom("Invalid overlay type format"))
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash)]
pub struct BundleContent {
    pub said: ReferenceAttrType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq)]
pub struct CaptureContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<IndexMap<String, NestedAttrType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IndexMap<String, NestedValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flagged_attributes: Option<Vec<String>>,
}

// TODO remove it when moved all to BaseAttributeContent
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<IndexMap<String, NestedValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IndexMap<String, NestedValue>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash)]
#[serde(untagged)]
/// Enum representing type supported in bundle (From command)
///
/// References: supports ref said and ref name
pub enum ReferenceAttrType {
    Reference(RefValue),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq)]
#[serde(untagged)]
pub enum NestedValue {
    Reference(RefValue),
    Value(String),
    Object(IndexMap<String, NestedValue>),
    Array(Vec<NestedValue>),
}
impl NestedValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            NestedValue::Reference(ref_value) => {
                ref_value.hash(state);
            }
            NestedValue::Value(value) => {
                value.hash(state);
            }
            NestedValue::Object(object) => {
                for (key, value) in object {
                    key.hash(state);
                    value.hash(state);
                }
            }
            NestedValue::Array(array) => {
                for value in array {
                    value.hash(state);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum RefValue {
    Said(said::SelfAddressingIdentifier),
    // This type is supported only for local-reference feature from facade (oca)
    Name(String),
}

impl FromStr for RefValue {
    type Err = RefValueParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tag, rest) = s
            .split_once(':')
            .ok_or(RefValueParsingError::MissingColon)?;
        match tag {
            "refs" => {
                let said = SelfAddressingIdentifier::from_str(rest)?;
                Ok(RefValue::Said(said))
            }
            "refn" => Ok(RefValue::Name(rest.to_string())),
            _ => Err(RefValueParsingError::UnknownTag(tag.to_string())),
        }
    }
}

#[derive(Error, Debug)]

pub enum RefValueParsingError {
    #[error("Missing colon")]
    MissingColon,
    #[error("Unknown tag `{0}`. Referece need to start with `refs` od `refn`")]
    UnknownTag(String),
    #[error("Invalid said: {0}")]
    SaidError(#[from] said::error::Error),
}

impl fmt::Display for RefValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RefValue::Said(said) => write!(f, "refs:{}", said),
            RefValue::Name(name) => write!(f, "refn:{}", name),
        }
    }
}
impl Serialize for RefValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            RefValue::Said(said) => serializer.serialize_str(format!("refs:{}", said).as_str()),
            RefValue::Name(name) => serializer.serialize_str(format!("refn:{}", name).as_str()),
        }
    }
}

// Implement Deserialize for Command
impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Kind,
            ObjectKind,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`type` or `object_kind`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Kind),
                            "object_kind" => Ok(Field::ObjectKind),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct CommandVisitor;

        impl<'de> Visitor<'de> for CommandVisitor {
            type Value = Command;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Command")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Command, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut kind = None;
                let mut object_kind = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Kind => {
                            if kind.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            kind = Some(map.next_value()?);
                        }
                        Field::ObjectKind => {
                            if object_kind.is_some() {
                                return Err(de::Error::duplicate_field("object_kind"));
                            }
                            let object_kind_str: String = map.next_value()?;
                            match object_kind_str.as_str() {
                                "CaptureBase" => {
                                    // take the key frist otherwise next value would not work
                                    // properly
                                    let _content_key: Option<String> = map.next_key()?;
                                    let content: CaptureContent = map.next_value()?;
                                    object_kind = Some(ObjectKind::CaptureBase(content));
                                }
                                "OCABundle" => {
                                    // take the key frist otherwise next value would not work
                                    // properly
                                    let _content_key: Option<String> = map.next_key()?;
                                    let content: BundleContent = map.next_value()?;
                                    object_kind = Some(ObjectKind::OCABundle(content));
                                }
                                _ => {
                                    // take the key frist otherwise next value would not work
                                    // properly
                                    let _content_key: Option<String> = map.next_key()?;
                                    // if it is not a CaptureBase or OCABundle, it must be an Overlay
                                    let overlay_type =
                                        OverlayType::from_str(object_kind_str.as_str());
                                    match overlay_type {
                                        Ok(overlay_type) => {
                                            let content: Content = map.next_value()?;
                                            object_kind =
                                                Some(ObjectKind::Overlay(overlay_type, content));
                                        }
                                        Err(_) => {
                                            return Err(de::Error::unknown_field(
                                                &object_kind_str,
                                                VARIANTS,
                                            ))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                let kind = kind.ok_or_else(|| de::Error::missing_field("type"))?;
                let object_kind =
                    object_kind.ok_or_else(|| de::Error::missing_field("object_kind"))?;

                Ok(Command { kind, object_kind })
            }
        }

        const FIELDS: &[&str] = &["type", "object_kind", "content"];
        const VARIANTS: &[&str] = &["CaptureBase", "OCABundle", "Overlay"];
        deserializer.deserialize_struct("Command", FIELDS, CommandVisitor)
    }
}

impl<'de> Deserialize<'de> for RefValue {
    fn deserialize<D>(deserializer: D) -> Result<RefValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let (tag, rest) = s.split_once(':').ok_or(serde::de::Error::custom(format!(
            "invalid reference: {}",
            s
        )))?;
        match tag {
            "refs" => {
                let said = SelfAddressingIdentifier::from_str(rest);
                match said {
                    Ok(said) => Ok(RefValue::Said(said)),
                    Err(_) => Err(serde::de::Error::custom(format!(
                        "invalid reference: {}",
                        s
                    ))),
                }
            }
            "refn" => Ok(RefValue::Name(rest.to_string())),
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
            meta: HashMap::new(),
        }
    }
}

impl Default for OCAAst {
    fn default() -> Self {
        Self::new()
    }
}

/*
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
}*/

impl From<u8> for ObjectKind {
    fn from(val: u8) -> Self {
        match val {
            0 => ObjectKind::CaptureBase(CaptureContent {
                attributes: None,
                properties: None,
                flagged_attributes: None,
            }),
            1 => ObjectKind::OCABundle(BundleContent {
                said: ReferenceAttrType::Reference(RefValue::Name("".to_string())),
            }),
            2 => ObjectKind::Overlay(
                OverlayType::Label,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            3 => ObjectKind::Overlay(
                OverlayType::Information,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            4 => ObjectKind::Overlay(
                OverlayType::Encoding,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            5 => ObjectKind::Overlay(
                OverlayType::CharacterEncoding,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            6 => ObjectKind::Overlay(
                OverlayType::Format,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            7 => ObjectKind::Overlay(
                OverlayType::Meta,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            8 => ObjectKind::Overlay(
                OverlayType::Standard,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            9 => ObjectKind::Overlay(
                OverlayType::Cardinality,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            10 => ObjectKind::Overlay(
                OverlayType::Conditional,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            11 => ObjectKind::Overlay(
                OverlayType::Conformance,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            12 => ObjectKind::Overlay(
                OverlayType::EntryCode,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            13 => ObjectKind::Overlay(
                OverlayType::Entry,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            14 => ObjectKind::Overlay(
                OverlayType::Unit,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            15 => ObjectKind::Overlay(
                OverlayType::AttributeMapping,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            16 => ObjectKind::Overlay(
                OverlayType::EntryCodeMapping,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            17 => ObjectKind::Overlay(
                OverlayType::Subset,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            18 => ObjectKind::Overlay(
                OverlayType::UnitMapping,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            19 => ObjectKind::Overlay(
                OverlayType::Layout,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            20 => ObjectKind::Overlay(
                OverlayType::Sensitivity,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            21 => ObjectKind::Overlay(
                OverlayType::Link,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            22 => ObjectKind::Overlay(
                OverlayType::AttributeFraming,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
            _ => panic!("Unknown object type"),
        }
    }
}

impl From<ObjectKind> for u8 {
    fn from(val: ObjectKind) -> Self {
        match val {
            ObjectKind::CaptureBase(_) => 0,
            ObjectKind::OCABundle(_) => 1,
            ObjectKind::Overlay(OverlayType::Label, _) => 2,
            ObjectKind::Overlay(OverlayType::Information, _) => 3,
            ObjectKind::Overlay(OverlayType::Encoding, _) => 4,
            ObjectKind::Overlay(OverlayType::CharacterEncoding, _) => 5,
            ObjectKind::Overlay(OverlayType::Format, _) => 6,
            ObjectKind::Overlay(OverlayType::Meta, _) => 7,
            ObjectKind::Overlay(OverlayType::Standard, _) => 8,
            ObjectKind::Overlay(OverlayType::Cardinality, _) => 9,
            ObjectKind::Overlay(OverlayType::Conditional, _) => 10,
            ObjectKind::Overlay(OverlayType::Conformance, _) => 11,
            ObjectKind::Overlay(OverlayType::EntryCode, _) => 12,
            ObjectKind::Overlay(OverlayType::Entry, _) => 13,
            ObjectKind::Overlay(OverlayType::Unit, _) => 14,
            ObjectKind::Overlay(OverlayType::AttributeMapping, _) => 15,
            ObjectKind::Overlay(OverlayType::EntryCodeMapping, _) => 16,
            ObjectKind::Overlay(OverlayType::Subset, _) => 17,
            ObjectKind::Overlay(OverlayType::UnitMapping, _) => 18,
            ObjectKind::Overlay(OverlayType::Layout, _) => 19,
            ObjectKind::Overlay(OverlayType::Sensitivity, _) => 20,
            ObjectKind::Overlay(OverlayType::Link, _) => 21,
            ObjectKind::Overlay(OverlayType::AttributeFraming, _) => 22,
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
            "CaptureBase" => Ok(ObjectKind::CaptureBase(CaptureContent {
                attributes: None,
                properties: None,
                flagged_attributes: None,
            })),
            "OCABundle" => Ok(ObjectKind::OCABundle(BundleContent {
                said: ReferenceAttrType::Reference(RefValue::Name("".to_string())),
            })),
            "Label" => Ok(ObjectKind::Overlay(
                OverlayType::Label,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Information" => Ok(ObjectKind::Overlay(
                OverlayType::Information,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Encoding" => Ok(ObjectKind::Overlay(
                OverlayType::Encoding,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "CharacterEncoding" => Ok(ObjectKind::Overlay(
                OverlayType::CharacterEncoding,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Format" => Ok(ObjectKind::Overlay(
                OverlayType::Format,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Meta" => Ok(ObjectKind::Overlay(
                OverlayType::Meta,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Standard" => Ok(ObjectKind::Overlay(
                OverlayType::Standard,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Cardinality" => Ok(ObjectKind::Overlay(
                OverlayType::Cardinality,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Conditional" => Ok(ObjectKind::Overlay(
                OverlayType::Conditional,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Conformance" => Ok(ObjectKind::Overlay(
                OverlayType::Conformance,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "EntryCode" => Ok(ObjectKind::Overlay(
                OverlayType::EntryCode,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Entry" => Ok(ObjectKind::Overlay(
                OverlayType::Entry,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Unit" => Ok(ObjectKind::Overlay(
                OverlayType::Unit,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "AttributeMapping" => Ok(ObjectKind::Overlay(
                OverlayType::AttributeMapping,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "EntryCodeMapping" => Ok(ObjectKind::Overlay(
                OverlayType::EntryCodeMapping,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Subset" => Ok(ObjectKind::Overlay(
                OverlayType::Subset,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "UnitMapping" => Ok(ObjectKind::Overlay(
                OverlayType::UnitMapping,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Layout" => Ok(ObjectKind::Overlay(
                OverlayType::Layout,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Sensitivity" => Ok(ObjectKind::Overlay(
                OverlayType::Sensitivity,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "Link" => Ok(ObjectKind::Overlay(
                OverlayType::Link,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            "AttributeFraming" => Ok(ObjectKind::Overlay(
                OverlayType::AttributeFraming,
                Content {
                    attributes: None,
                    properties: None,
                },
            )),
            _ => Err(serde::de::Error::custom(format!(
                "unknown object kind: {}",
                s
            ))),
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
        let mut flagged_attributes = Vec::new();

        let arr = NestedAttrType::Array(Box::new(NestedAttrType::Value(AttributeType::Boolean)));
        attributes.insert("allowed".to_string(), arr);
        attributes.insert(
            "test".to_string(),
            NestedAttrType::Value(AttributeType::Text),
        );

        flagged_attributes.push("test".to_string());
        properties.insert("test".to_string(), NestedValue::Value("test".to_string()));
        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(attributes),
                properties: Some(properties),
                flagged_attributes: flagged_attributes.into(),
            }),
        };

        let lable_command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::Overlay(
                OverlayType::Label,
                Content {
                    attributes: None,
                    properties: None,
                },
            ),
        };

        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        ocaast.commands.push(lable_command);

        let serialized = serde_json::to_string(&ocaast).unwrap();
        assert_eq!(
            serialized,
            r#"{"version":"1.0.0","commands":[{"type":"Add","object_kind":"CaptureBase","content":{"attributes":{"allowed":["Boolean"],"test":"Text"},"properties":{"test":"test"},"flagged_attributes":["test"]}},{"type":"Add","object_kind":"Label","content":{}}],"commands_meta":{},"meta":{}}"#
        );

        let deser: OCAAst = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ocaast, deser);
    }
}
