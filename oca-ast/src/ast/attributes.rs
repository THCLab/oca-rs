use indexmap::IndexMap;
use serde::{Serialize, Deserialize, Deserializer, de::{Visitor, self}};
use std::{hash::Hash, fmt, str::FromStr};

use super::{RefValue, AttributeType};


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


            // For string we have based types and references
            fn visit_str<E>(self, value: &str) -> Result<NestedAttrType, E>
            where
                E: de::Error,
            {
                // Try to parse base attribute first and then references
                match AttributeType::from_str(value) {
                    Ok(attr_type) => Ok(NestedAttrType::Value(attr_type)),
                    Err(_) => {
                        match RefValue::from_str(value) {
                            Ok(ref_value) => Ok(NestedAttrType::Reference(ref_value)),
                            Err(_) => Err(de::Error::custom(format!("invalid reference: {}", value)))
                        }
                    }
                }
            }

            // Example for one of the visit methods
            fn visit_map<V>(self, _: V) -> Result<NestedAttrType, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                const MAX_DEPTH: usize = 4;
                if self.depth > MAX_DEPTH {
                    return Err(de::Error::custom("recursion depth exceeded"));
                }

                let object = IndexMap::new();
                Ok(NestedAttrType::Object(object))
            }
        }

        deserializer.deserialize_any(NestedAttrTypeVisitor { depth: 0 })
    }
}
