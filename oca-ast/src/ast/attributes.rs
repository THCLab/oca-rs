use recursion::ExpandableExt;
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use std::hash::Hash;
use wasm_bindgen::JsValue;

use super::{
    error::AttributeError,
    recursive_attributes::{AttributeTypeResult, NestedAttrTypeFrame},
    AttributeType, RefValue,
};

#[derive(Debug, PartialEq, Clone, Eq, Serialize)]
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
    #[serde(serialize_with = "array_serializer")]
    Array(Box<NestedAttrType>),
    /// Indicator that attribute was removed and does not need any type
    Null,
}

impl NestedAttrType {
    pub fn to_js_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(self)?)
    }

    pub fn from_js_value(value: JsValue) -> Result<Self, JsValue> {
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}

fn array_serializer<S>(arr: &NestedAttrType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Serialize the inner value as an array
    let mut seq = serializer.serialize_seq(Some(1))?;
    seq.serialize_element(&arr)?;
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
            NestedAttrType::Array(array) => {
                array.hash(state);
            }
            NestedAttrType::Null => {
                "null".hash(state);
            }
        }
    }
}

impl<'de> Deserialize<'de> for NestedAttrType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input: serde_json::Value = serde_json::Value::deserialize(deserializer)?;

        let expanded = AttributeTypeResult::expand_frames(input, |seed| match seed {
            serde_json::Value::String(text) => match &text.get(..5) {
                Some("refs:") | Some("refn:") => {
                    let res = text.parse::<RefValue>();
                    match res {
                        Ok(ref_value) => NestedAttrTypeFrame::Reference(ref_value).into(),
                        Err(e) => AttributeError::from(e).into(),
                    }
                }
                _ => {
                    let res = text.parse::<AttributeType>();
                    match res {
                        Ok(attr_type) => NestedAttrTypeFrame::Value(attr_type).into(),
                        Err(_) => AttributeError::UnknownAttributeType(text).into(),
                    }
                }
            },
            serde_json::Value::Array(arr) => NestedAttrTypeFrame::Array(arr[0].clone()).into(),
            value => {
                AttributeError::ConvertingFailure(serde_json::to_string(&value).unwrap()).into()
            }
        });
        match expanded.value() {
            Ok(el) => Ok(el),
            Err(er) => Err(serde::de::Error::custom(er)),
        }
    }
}

#[cfg(test)]
mod tests {
    use said::derivation::{HashFunction, HashFunctionCode};

    use crate::ast::{error::AttributeError, AttributeType, NestedAttrType, RefValue};

    #[test]
    fn test_nested_array_attribute_type_serialization() {
        let arr = NestedAttrType::Array(Box::new(NestedAttrType::Array(Box::new(
            NestedAttrType::Value(AttributeType::Boolean),
        ))));
        let said = HashFunction::from(HashFunctionCode::Blake3_256).derive("fff".as_bytes());

        let serialized = serde_json::to_string(&arr).unwrap();
        let expected = r#"[["Boolean"]]"#;
        assert_eq!(expected, serialized);

        let deser: NestedAttrType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(arr, deser);

        let attributes =
            NestedAttrType::Array(Box::new(NestedAttrType::Reference(RefValue::Said(said))));

        let serialized = serde_json::to_string(&attributes).unwrap();
        let expected = r#"["refs:EEokfxxqwAM08iku7VHMaVFBaEGYVi2W-ctBKaTW6QdJ"]"#;
        assert_eq!(expected, serialized);

        let deser: NestedAttrType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(attributes, deser);
    }

    #[test]
    fn test_nested_attribute_deserialize() {
        let serialized = r#"["Numeric"]"#;
        let deser: NestedAttrType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            NestedAttrType::Array(Box::new(NestedAttrType::Value(AttributeType::Numeric))),
            deser
        );

        let wrong_type = r#"["Wrong"]"#;
        let deser = serde_json::from_str::<NestedAttrType>(&wrong_type);
        assert!(deser.is_err());
        assert_eq!(
            deser.unwrap_err().to_string(),
            AttributeError::UnknownAttributeType("Wrong".to_string()).to_string()
        );

        let serialized = r#"["refs:EEokfxxqwAM08iku7VHMaVFBaEGYVi2W-ctBKaTW6QdJ"]"#;
        let expected = NestedAttrType::Array(Box::new(NestedAttrType::Reference(RefValue::Said(
            said::derivation::HashFunction::from(said::derivation::HashFunctionCode::Blake3_256)
                .derive("fff".as_bytes()),
        ))));
        let deser: NestedAttrType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(expected, deser);

        let serialized = r#"["refn:bob"]"#;
        let expected = NestedAttrType::Array(Box::new(NestedAttrType::Reference(RefValue::Name(
            "bob".to_string(),
        ))));
        let deser: NestedAttrType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(expected, deser);
    }

    #[test]
    fn test_reference_attribute_deserialize() {
        let wrong_said = r#"["refs:wrong_said"]"#;
        let deser = serde_json::from_str::<NestedAttrType>(&wrong_said);
        assert_eq!(deser.unwrap_err().to_string(), "Invalid said: Unknown code");

        let missing_ref_tag = r#"["EEokfxxqwAM08iku7VHMaVFBaEGYVi2W-ctBKaTW6QdJ"]"#;
        let deser = serde_json::from_str::<NestedAttrType>(&missing_ref_tag);
        assert_eq!(
            deser.unwrap_err().to_string(),
            "Attribute type EEokfxxqwAM08iku7VHMaVFBaEGYVi2W-ctBKaTW6QdJ doesn't exist"
        );
    }
}
