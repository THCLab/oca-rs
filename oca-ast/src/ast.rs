use indexmap::IndexMap;
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::{Visitor, self}};
use strum_macros::Display;
use std::{str::FromStr, collections::HashMap, fmt};
use wasm_bindgen::prelude::*;
use std::hash::{Hash, Hasher};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OCAAst {
    pub version: String,
    pub commands: Vec<Command>,
    pub commands_meta: IndexMap<usize, CommandMeta>,
    pub meta: HashMap<String, String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, PartialEq, Clone, Eq)]
#[serde(tag = "object_kind", content = "content")]
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
            ObjectKind::Overlay(overlay_type, content) => {
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
            _ => {
                Err(())
            }
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


// TODO implement deserializer for NestedAttrType
impl<'de> Deserialize<'de> for NestedAttrType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NestedAttrTypeVisitor {
            depth: usize,
        }

        impl<'de> Visitor<'de> for NestedAttrTypeVisitor {
            type Value = NestedAttrType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid NestedAttrType")
            }

            // Implement the visit_* methods to handle each case
            // ...


            fn visit_str<E>(self, value: &str) -> Result<NestedAttrType, E>
            where
                E: de::Error,
            {
                println!("visit_str: {}", value);
                // Try to parse base attribute
                match AttributeType::from_str(value) {
                    Ok(attr_type) => Ok(NestedAttrType::Value(attr_type)),
                    Err(_) => {
                        Ok(NestedAttrType::Reference(RefValue::from_str(value).unwrap()))
                    }
                }
            }

            // Example for one of the visit methods
            fn visit_map<V>(self, mut map: V) -> Result<NestedAttrType, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                const MAX_DEPTH: usize = 4;
                if self.depth > MAX_DEPTH {
                    return Err(de::Error::custom("recursion depth exceeded"));
                }

                let mut object = IndexMap::new();
                println!("depth: {}", self.depth);
                println!(">>>>> object: {:?}", object);
                Ok(NestedAttrType::Object(object))
            }
        }

        deserializer.deserialize_any(NestedAttrTypeVisitor { depth: 0 })
    }
}


#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
#[serde(untagged)]
/// Enum representing attribute type which can be nested.
///
/// References: supports ref said and ref name
/// Value: supports all AttributeType
/// Object: can be inline object which can have nested attributes types
/// Array: is an array of specific type (only one type allowed)
pub enum NestedAttrType {
    Reference(RefValue),
    Value(AttributeType),
    Object(IndexMap<String, NestedAttrType>),
    Array(Box<NestedAttrType>),
    /// Indicator that attribute was removed and does not need any type
    Null,
}

impl Hash for NestedAttrType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            NestedAttrType::Reference(ref_value) => {
                ref_value.hash(state);
            }
            NestedAttrType::Value(attr_type) => {
                attr_type.hash(state);
            }
            NestedAttrType::Object(object) => {
                for (key, value) in object {
                    key.hash(state);
                    value.hash(state);
                }
            }
            NestedAttrType::Array(array) => {
                array.hash(state);
            }
            NestedAttrType::Null => {
                "null".hash(state);
            }
        }
    }
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
    Said(String),
    Name(String),
}


impl FromStr for RefValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tag, rest) = s.split_once(':').ok_or(())?;
        match tag {
            "refs" => Ok(RefValue::Said(rest.to_string())),
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
            RefValue::Said(said) => serializer.serialize_str(
                format!("refs:{}", said).as_str()
            ),
            RefValue::Name(name) => serializer.serialize_str(
                format!("refn:{}", name).as_str()
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
            "refs" => Ok(RefValue::Said(rest.to_string())),
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
            1 => ObjectKind::OCABundle(BundleContent { said: ReferenceAttrType::Reference(RefValue::Said("".to_string())) }),
            2 => ObjectKind::Overlay(OverlayType::Label, Content { attributes: None, properties: None }),
            3 => ObjectKind::Overlay(OverlayType::Information, Content { attributes: None, properties: None }),
            4 => ObjectKind::Overlay(OverlayType::Encoding, Content { attributes: None, properties: None }),
            5 => ObjectKind::Overlay(OverlayType::CharacterEncoding, Content { attributes: None, properties: None }),
            6 => ObjectKind::Overlay(OverlayType::Format, Content { attributes: None, properties: None }),
            7 => ObjectKind::Overlay(OverlayType::Meta, Content { attributes: None, properties: None }),
            8 => ObjectKind::Overlay(OverlayType::Standard, Content { attributes: None, properties: None }),
            9 => ObjectKind::Overlay(OverlayType::Cardinality, Content { attributes: None, properties: None }),
            10 => ObjectKind::Overlay(OverlayType::Conditional, Content { attributes: None, properties: None }),
            11 => ObjectKind::Overlay(OverlayType::Conformance, Content { attributes: None, properties: None }),
            12 => ObjectKind::Overlay(OverlayType::EntryCode, Content { attributes: None, properties: None }),
            13 => ObjectKind::Overlay(OverlayType::Entry, Content { attributes: None, properties: None }),
            14 => ObjectKind::Overlay(OverlayType::Unit, Content { attributes: None, properties: None }),
            15 => ObjectKind::Overlay(OverlayType::AttributeMapping, Content { attributes: None, properties: None }),
            16 => ObjectKind::Overlay(OverlayType::EntryCodeMapping, Content { attributes: None, properties: None }),
            17 => ObjectKind::Overlay(OverlayType::Subset, Content { attributes: None, properties: None }),
            18 => ObjectKind::Overlay(OverlayType::UnitMapping, Content { attributes: None, properties: None }),
            19 => ObjectKind::Overlay(OverlayType::Layout, Content { attributes: None, properties: None }),
            20 => ObjectKind::Overlay(OverlayType::Sensitivity, Content { attributes: None, properties: None }),
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
            "OCABundle" => Ok(ObjectKind::OCABundle(BundleContent { said: ReferenceAttrType::Reference(RefValue::Said("".to_string())) })),
            "Label" => Ok(ObjectKind::Overlay(OverlayType::Label, Content { attributes: None, properties: None })),
            "Information" => Ok(ObjectKind::Overlay(OverlayType::Information, Content { attributes: None, properties: None })),
            "Encoding" => Ok(ObjectKind::Overlay(OverlayType::Encoding, Content { attributes: None, properties: None })),
            "CharacterEncoding" => Ok(ObjectKind::Overlay(OverlayType::CharacterEncoding, Content { attributes: None, properties: None })),
            "Format" => Ok(ObjectKind::Overlay(OverlayType::Format, Content { attributes: None, properties: None })),
            "Meta" => Ok(ObjectKind::Overlay(OverlayType::Meta, Content { attributes: None, properties: None })),
            "Standard" => Ok(ObjectKind::Overlay(OverlayType::Standard, Content { attributes: None, properties: None })),
            "Cardinality" => Ok(ObjectKind::Overlay(OverlayType::Cardinality, Content { attributes: None, properties: None })),
            "Conditional" => Ok(ObjectKind::Overlay(OverlayType::Conditional, Content { attributes: None, properties: None })),
            "Conformance" => Ok(ObjectKind::Overlay(OverlayType::Conformance, Content { attributes: None, properties: None })),
            "EntryCode" => Ok(ObjectKind::Overlay(OverlayType::EntryCode, Content { attributes: None, properties: None })),
            "Entry" => Ok(ObjectKind::Overlay(OverlayType::Entry, Content { attributes: None, properties: None })),
            "Unit" => Ok(ObjectKind::Overlay(OverlayType::Unit, Content { attributes: None, properties: None })),
            "AttributeMapping" => Ok(ObjectKind::Overlay(OverlayType::AttributeMapping, Content { attributes: None, properties: None })),
            "EntryCodeMapping" => Ok(ObjectKind::Overlay(OverlayType::EntryCodeMapping, Content { attributes: None, properties: None })),
            "Subset" => Ok(ObjectKind::Overlay(OverlayType::Subset, Content { attributes: None, properties: None })),
            "UnitMapping" => Ok(ObjectKind::Overlay(OverlayType::UnitMapping, Content { attributes: None, properties: None })),
            "Layout" => Ok(ObjectKind::Overlay(OverlayType::Layout, Content { attributes: None, properties: None })),
            "Sensitivity" => Ok(ObjectKind::Overlay(OverlayType::Sensitivity, Content { attributes: None, properties: None })),
            _ => Err(serde::de::Error::custom(format!("unknown object kind: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use log::debug;

    use super::*;

    #[test]
    fn test_ocaast_serialize() {
        let mut attributes = IndexMap::new();
        let mut properties = IndexMap::new();
        let mut person = IndexMap::new();
        person.insert("name".to_string(), NestedAttrType::Value(AttributeType::Text));

        attributes.insert("test".to_string(), NestedAttrType::Value(AttributeType::Text));
        attributes.insert("person".to_string(), NestedAttrType::Object(person));
        properties.insert("test".to_string(), NestedValue::Value("test".to_string()));
        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase(CaptureContent {
                attributes: Some(attributes),
                properties: Some(properties),
            }),
        };

        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        let serialized = serde_json::to_string(&ocaast).unwrap();
        debug!("serialized: {}", serialized);
        assert_eq!(
            serialized,
            r#"{"version":"1.0.0","commands":[{"type":"Add","object_kind":"CaptureBase","content":{"attributes":{"test":"Text","person":{"name":"Text"}},"properties":{"test":"test"}}}],"commands_meta":{},"meta":{}}"#
        );
    }
}
