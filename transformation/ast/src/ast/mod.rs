use indexmap::IndexMap;
use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::hash::Hash;
use std::{collections::HashMap, fmt};

pub mod error;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TransformationAST {
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
    Rename,
    Link,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum ObjectKind {
    // Transformation(TransformationType),
    Rename(RenameContent),
    Link(LinkContent),
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum TransformationType {
    Rename(RenameContent),
    Link(LinkContent),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq)]
pub struct RenameContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<IndexMap<String, String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq)]
pub struct LinkContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<IndexMap<String, String>>,
}

impl Hash for ObjectKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ObjectKind::Rename(content) => {
                content.hash(state);
            }
            ObjectKind::Link(content) => {
                content.hash(state);
            }
        }
    }
}

impl Hash for RenameContent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(attributes) = &self.attributes {
            for (key, value) in attributes {
                key.hash(state);
                value.hash(state);
            }
        }
    }
}

impl Hash for LinkContent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(attributes) = &self.attributes {
            for (key, value) in attributes {
                key.hash(state);
                value.hash(state);
            }
        }
    }
}

/* impl ObjectKind {
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
} */

impl Serialize for ObjectKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ObjectKind", 3)?;
        match self {
            ObjectKind::Rename(content) => {
                state.serialize_field("object_kind", "Rename")?;
                state.serialize_field("content", content)?;
            }
            ObjectKind::Link(content) => {
                state.serialize_field("object_kind", "Link")?;
                state.serialize_field("content", content)?;
            }
        }
        state.end()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq)]
#[serde(untagged)]
pub enum NestedValue {
    Value(String),
    Object(IndexMap<String, NestedValue>),
    Array(Vec<NestedValue>),
}
impl NestedValue {
    /* fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
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
    } */
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
                                "Rename" => {
                                    // take the key frist otherwise next value would not work
                                    // properly
                                    let _content_key: Option<String> = map.next_key()?;
                                    let content: RenameContent = map.next_value()?;
                                    object_kind = Some(ObjectKind::Rename(content));
                                }
                                "Link" => {
                                    // take the key frist otherwise next value would not work
                                    // properly
                                    let _content_key: Option<String> = map.next_key()?;
                                    let content: LinkContent = map.next_value()?;
                                    object_kind = Some(ObjectKind::Link(content));
                                }
                                _ => {}
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
        const _VARIANTS: &[&str] = &["Rename"];
        deserializer.deserialize_struct("Command", FIELDS, CommandVisitor)
    }
}

impl TransformationAST {
    pub fn new() -> Self {
        TransformationAST {
            // Version of OCA specification
            version: String::from("1.0.0"),
            commands: Vec::new(),
            commands_meta: IndexMap::new(),
            meta: HashMap::new(),
        }
    }
}

impl Default for TransformationAST {
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
            0 => ObjectKind::Rename(RenameContent { attributes: None }),
            1 => ObjectKind::Link(LinkContent { attributes: None }),
            _ => panic!("Unknown object type"),
        }
    }
}

impl From<ObjectKind> for u8 {
    fn from(val: ObjectKind) -> Self {
        match val {
            ObjectKind::Rename(_) => 0,
            ObjectKind::Link(_) => 1,
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
            "Rename" => Ok(ObjectKind::Rename(RenameContent { attributes: None })),
            "Link" => Ok(ObjectKind::Link(LinkContent { attributes: None })),
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
    fn test_ast_serialize() {
        let mut attributes = IndexMap::new();

        attributes.insert("digest".to_string(), "d".to_string());
        let command = Command {
            kind: CommandType::Rename,
            object_kind: ObjectKind::Rename(RenameContent {
                attributes: Some(attributes),
            }),
        };

        let mut ast = TransformationAST::new();
        ast.commands.push(command);

        let serialized = serde_json::to_string(&ast).unwrap();
        assert_eq!(
            serialized,
            r#"{"version":"1.0.0","commands":[{"type":"Rename","object_kind":"Rename","content":{"attributes":{"digest":"d"}}}],"commands_meta":{},"meta":{}}"#
        );

        let deser: TransformationAST = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ast, deser);
    }
}
