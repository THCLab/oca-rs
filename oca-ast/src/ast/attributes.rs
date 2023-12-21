use indexmap::IndexMap;
use recursion::{
    Collapsible, CollapsibleExt, Expandable, ExpandableExt, MappableFrame, PartiallyApplied,
};
use serde::{
    // de::{self, Visitor},
    ser::SerializeSeq,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::{collections::HashMap, hash::Hash};

use super::{AttributeType, RefValue};

#[derive(Debug, PartialEq, Clone, Eq, Serialize)]
#[serde(untagged)]
/// Enum representing attribute type which can be nested.
///
/// References: supports ref said and ref name
/// Value: supports all AttributeType
/// Object: can be inline object which can have nested attributes types
/// Array: is an array of specific type (only one type allowed)
pub enum NestedAttrType {
    #[serde(serialize_with = "array_serializer")]
    Array(Box<NestedAttrType>),
    Reference(RefValue),
    Value(AttributeType),
    Object(IndexMap<String, NestedAttrType>),
    /// Indicator that attribute was removed and does not need any type
    Null,
}

fn array_serializer<S>(foo: &Box<NestedAttrType>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Serialize the inner value as an array
    let mut seq = serializer.serialize_seq(Some(1))?;
    seq.serialize_element(&foo)?;
    seq.end()

    // Serialize the inner value and combine it with "Array"
    // let serialized = serde_json::to_string(&foo).unwrap();
    // let serialized_with_array = format!("Array[{}]", serialized);
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

pub enum NestedAttrTypeFrame<A> {
    Reference(RefValue),
    Value(AttributeType),
    Object(IndexMap<String, A>),
    Array(A),
    Null,
}

impl MappableFrame for NestedAttrTypeFrame<PartiallyApplied> {
    type Frame<X> = NestedAttrTypeFrame<X>;

    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            NestedAttrTypeFrame::Reference(reference) => NestedAttrTypeFrame::Reference(reference),
            NestedAttrTypeFrame::Value(val) => NestedAttrTypeFrame::Value(val),
            NestedAttrTypeFrame::Object(obj) => {
                let obj = obj
                    .into_iter()
                    .map(|(key, value)| (key, f(value)))
                    .collect();
                NestedAttrTypeFrame::Object(obj)
            }
            NestedAttrTypeFrame::Array(t) => NestedAttrTypeFrame::Array(f(t)),
            NestedAttrTypeFrame::Null => NestedAttrTypeFrame::Null,
        }
    }
}

impl Expandable for NestedAttrType {
    type FrameToken = NestedAttrTypeFrame<PartiallyApplied>;

    fn from_frame(val: <Self::FrameToken as MappableFrame>::Frame<Self>) -> Self {
        match val {
            NestedAttrTypeFrame::Reference(reference) => NestedAttrType::Reference(reference),
            NestedAttrTypeFrame::Value(v) => NestedAttrType::Value(v),
            NestedAttrTypeFrame::Object(obj) => NestedAttrType::Object(obj),
            NestedAttrTypeFrame::Array(arr) => NestedAttrType::Array(Box::new(arr)),
            NestedAttrTypeFrame::Null => NestedAttrType::Null,
        }
    }
}

impl Collapsible for NestedAttrType {
    type FrameToken = NestedAttrTypeFrame<PartiallyApplied>;

    fn into_frame(self) -> <Self::FrameToken as MappableFrame>::Frame<Self> {
        match self {
            NestedAttrType::Reference(reference) => NestedAttrTypeFrame::Reference(reference),
            NestedAttrType::Value(val) => NestedAttrTypeFrame::Value(val),
            NestedAttrType::Object(obj) => NestedAttrTypeFrame::Object(obj),
            NestedAttrType::Array(arr) => NestedAttrTypeFrame::Array(*arr),
            NestedAttrType::Null => NestedAttrTypeFrame::Null,
        }
    }
}

fn format_reference(ref_value: RefValue, references: &Option<HashMap<String, String>>) -> String {
    match ref_value {
        RefValue::Name(ref refn) => match references {
            Some(ref references) => {
                if let Some(refs) = references.get(refn) {
                    format!("refn:{}", refs)
                } else {
                    panic!("Reference not found: {}", refn)
                }
            }
            None => {
                format!("{}", refn)
            }
        },
        RefValue::Said(refs) => {
            format!("refs:{}", refs)
        }
    }
}

pub fn oca_file_format(
    nested: NestedAttrType,
    references: &Option<HashMap<String, String>>,
) -> String {
    nested.collapse_frames(|frame| match frame {
        NestedAttrTypeFrame::Reference(ref_value) => format_reference(ref_value, references),
        NestedAttrTypeFrame::Value(value) => {
            format!("{}", value)
        }
        NestedAttrTypeFrame::Object(obj) => {
            let start = "Object {".to_string();
            let end = "}".to_string();
            let inner_data = obj
                .into_iter()
                .map(|(obj_key, obj_value)| format!(" {}={}", obj_key, obj_value));
            let out = inner_data.collect::<Vec<_>>().join(", ");
            vec![start, out, end].join("")
        }
        NestedAttrTypeFrame::Array(arr) => {
            format!("Array[{}]", arr)
        }
        NestedAttrTypeFrame::Null => "".to_string(),
    })
}

impl<'de> Deserialize<'de> for NestedAttrType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input: serde_json::Value = serde_json::Value::deserialize(deserializer)?;

        let expanded = NestedAttrType::expand_frames(input, |seed| match seed {
            serde_json::Value::String(text) => match text.parse::<RefValue>() {
                Ok(ref_value) => NestedAttrTypeFrame::Reference(ref_value),
                Err(_) => match text.parse::<AttributeType>() {
                    Ok(attribute_type) => NestedAttrTypeFrame::Value(attribute_type),
                    Err(_) => todo!(),
                },
            },
            serde_json::Value::Object(obj) => {
                let mut idx_map = IndexMap::new();
                for (key, value) in obj {
                    idx_map.insert(key, value);
                }
                NestedAttrTypeFrame::Object(idx_map)
            }
            serde_json::Value::Array(arr) => NestedAttrTypeFrame::Array(arr[0].clone()),
            _ => todo!(),
        });
        Ok(expanded)
    }
}

// // TODO implement deserializer for NestedAttrType
// impl<'de> Deserialize<'de> for NestedAttrType {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct NestedAttrTypeVisitor {
//             depth: usize,
//         }

//         impl<'de> Visitor<'de> for NestedAttrTypeVisitor {
//             type Value = NestedAttrType;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("a valid NestedAttrType")
//             }

//             // Implement the visit_* methods to handle each case
//             // ...

//             // For string we have based types and references
//             fn visit_str<E>(self, value: &str) -> Result<NestedAttrType, E>
//             where
//                 E: de::Error,
//             {
//                 // Try to parse base attribute first and then references
//                 match AttributeType::from_str(value) {
//                     Ok(attr_type) => Ok(NestedAttrType::Value(attr_type)),
//                     Err(_) => match RefValue::from_str(value) {
//                         Ok(ref_value) => Ok(NestedAttrType::Reference(ref_value)),
//                         Err(_) => Err(de::Error::custom(format!("invalid reference: {}", value))),
//                     },
//                 }
//             }

//             // Example for one of the visit methods
//             fn visit_map<V>(self, _: V) -> Result<NestedAttrType, V::Error>
//             where
//                 V: serde::de::MapAccess<'de>,
//             {
//                 const MAX_DEPTH: usize = 4;
//                 if self.depth > MAX_DEPTH {
//                     return Err(de::Error::custom("recursion depth exceeded"));
//                 }

//                 let object = IndexMap::new();
//                 Ok(NestedAttrType::Object(object))
//             }
//         }

//         deserializer.deserialize_any(NestedAttrTypeVisitor { depth: 0 })
//     }
// }

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use said::derivation::{HashFunction, HashFunctionCode};

    use crate::ast::{attributes::oca_file_format, AttributeType, NestedAttrType, RefValue};

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

        let out = oca_file_format(attr, &None);
        assert_eq!(out, "Array[Object { name=Text,  age=Numeric,  data=refs:EJeWVGxkqxWrdGi0efOzwg1YQK8FrA-ZmtegiVEtAVcu}]");
        println!("{}", out);
    }

    #[test]
    fn test_nested_attribute_serialize() {
        let mut object_example = IndexMap::new();
        let mut person = IndexMap::new();
        person.insert(
            "name".to_string(),
            NestedAttrType::Value(AttributeType::Text),
        );

        let arr = NestedAttrType::Array(Box::new(NestedAttrType::Value(AttributeType::Boolean)));
        object_example.insert("allowed".to_string(), arr);

        object_example.insert(
            "test".to_string(),
            NestedAttrType::Value(AttributeType::Text),
        );
        object_example.insert("person".to_string(), NestedAttrType::Object(person));
        let said = HashFunction::from(HashFunctionCode::Blake3_256).derive("fff".as_bytes());
        object_example.insert(
            "ref".to_string(),
            NestedAttrType::Reference(RefValue::Said(said.clone())),
        );

        let attributes = NestedAttrType::Object(object_example);

        let serialized = serde_json::to_string(&attributes).unwrap();
        let expected = r#"{"allowed":["Boolean"],"test":"Text","person":{"name":"Text"},"ref":"refs:EEokfxxqwAM08iku7VHMaVFBaEGYVi2W-ctBKaTW6QdJ"}"#;
        assert_eq!(expected, serialized);

        let deser: NestedAttrType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(attributes, deser);

        let attributes =
            NestedAttrType::Array(Box::new(NestedAttrType::Reference(RefValue::Said(said))));

        let serialized = serde_json::to_string(&attributes).unwrap();
        let expected = r#"["refs:EEokfxxqwAM08iku7VHMaVFBaEGYVi2W-ctBKaTW6QdJ"]"#;
        assert_eq!(expected, serialized);

        let deser: NestedAttrType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(attributes, deser);
    }
}
