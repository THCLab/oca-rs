use indexmap::IndexMap;
use said::SelfAddressingIdentifier;
use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::hash::Hash;
use std::{collections::HashMap, fmt, str::FromStr};
use strum_macros::Display;
use wasm_bindgen::prelude::*;

pub use self::attributes::NestedAttrType;

pub mod attributes;

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
        match &self.attributes {
            Some(attributes) => {
                for (key, value) in attributes {
                    key.hash(state);
                    value.hash(state);
                }
            }
            None => {}
        }
        match &self.properties {
            Some(properties) => {
                for (key, value) in properties {
                    key.hash(state);
                    value.hash(state);
                }
            }
            None => {}
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
}

impl Serialize for OverlayType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OverlayType::Label => serializer.serialize_str("spec/overlays/label/1.0"),
            OverlayType::Information => serializer.serialize_str("spec/overlays/information/1.0"),
            OverlayType::Encoding => serializer.serialize_str("spec/overlays/encoding/1.0"),
            OverlayType::CharacterEncoding => {
                serializer.serialize_str("spec/overlays/character_encoding/1.0")
            }
            OverlayType::Format => serializer.serialize_str("spec/overlays/format/1.0"),
            OverlayType::Meta => serializer.serialize_str("spec/overlays/meta/1.0"),
            OverlayType::Standard => serializer.serialize_str("spec/overlays/standard/1.0"),
            OverlayType::Cardinality => serializer.serialize_str("spec/overlays/cardinality/1.0"),
            OverlayType::Conditional => serializer.serialize_str("spec/overlays/conditional/1.0"),
            OverlayType::Conformance => serializer.serialize_str("spec/overlays/conformance/1.0"),
            OverlayType::EntryCode => serializer.serialize_str("spec/overlays/entry_code/1.0"),
            OverlayType::Entry => serializer.serialize_str("spec/overlays/entry/1.0"),
            OverlayType::Unit => serializer.serialize_str("spec/overlays/unit/1.0"),
            OverlayType::AttributeMapping => serializer.serialize_str("spec/overlays/mapping/1.0"),
            OverlayType::EntryCodeMapping => {
                serializer.serialize_str("spec/overlays/entry_code_mapping/1.0")
            }
            OverlayType::Subset => serializer.serialize_str("spec/overlays/subset/1.0"),
            OverlayType::UnitMapping => serializer.serialize_str("spec/overlays/unit_mapping/1.0"),
            OverlayType::Layout => serializer.serialize_str("spec/overlays/layout/1.0"),
            OverlayType::Sensitivity => serializer.serialize_str("spec/overlays/sensitivity/1.0"),
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
            "spec/overlays/character_encoding/1.0" => Ok(OverlayType::CharacterEncoding),
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
            "spec/overlays/entry_code_mapping/1.0" => Ok(OverlayType::EntryCodeMapping),
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tag, rest) = s.split_once(':').ok_or(())?;
        match tag {
            "refs" => {
                let said = SelfAddressingIdentifier::from_str(rest);
                match said {
                    Ok(said) => Ok(RefValue::Said(said)),
                    Err(_) => Err(()),
                }
            }
            "refn" => Ok(RefValue::Name(rest.to_string())),
            _ => Err(()),
        }
    }
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

                impl<'de> Visitor<'de> for FieldVisitor {
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

        const FIELDS: &'static [&'static str] = &["type", "object_kind", "content"];
        const VARIANTS: &'static [&'static str] = &["CaptureBase", "OCABundle", "Overlay"];
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
        let mut person = IndexMap::new();
        person.insert(
            "name".to_string(),
            NestedAttrType::Value(AttributeType::Text),
        );

        let arr = NestedAttrType::Array(Box::new(NestedAttrType::Value(AttributeType::Boolean)));
        attributes.insert("allowed".to_string(), arr);
        attributes.insert(
            "test".to_string(),
            NestedAttrType::Value(AttributeType::Text),
        );
        attributes.insert("person".to_string(), NestedAttrType::Object(person));
        properties.insert("test".to_string(), NestedValue::Value("test".to_string()));
        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(attributes),
                properties: Some(properties),
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
            r#"{"version":"1.0.0","commands":[{"type":"Add","object_kind":"CaptureBase","content":{"attributes":{"allowed":["Boolean"],"test":"Text","person":{"name":"Text"}},"properties":{"test":"test"}}},{"type":"Add","object_kind":"Label","content":{}}],"commands_meta":{},"meta":{}}"# // r#"{"version":"1.0.0","commands":[{"type":"Add","object_kind":"CaptureBase","content":{"attributes":{"test":"Text","person":{"name":"Text"}},"properties":{"test":"test"}}},{"type":"Add","object_kind":"Label","content":{}}],"commands_meta":{},"meta":{}}"#
        );

        let deser: OCAAst = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ocaast, deser);
    }
}
