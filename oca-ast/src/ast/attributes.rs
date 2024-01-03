use recursion::{
    Collapsible, Expandable, ExpandableExt, MappableFrame, PartiallyApplied,
};
use serde::{
    ser::SerializeSeq,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use wasm_bindgen::JsValue;
use std::hash::Hash;

use super::{AttributeType, RefValue, error::AttributeError, nested_result::{NestedResult, NestedResultFrame}};

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

// TODO using from_serde is deprecated needs to be replaced with serd-wasm-bindgen
impl NestedAttrType {
    pub fn to_js_value(&self) -> JsValue {
        JsValue::from_serde(self).unwrap()
    }

    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        value.into_serde().map_err(|e| JsValue::from_str(&format!("Error converting JsValue to NestedAttrType: {:?}", e)))

    }
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
    Array(A),
    Null,
}

impl MappableFrame for NestedAttrTypeFrame<PartiallyApplied> {
    type Frame<X> = NestedAttrTypeFrame<X>;

    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            NestedAttrTypeFrame::Reference(reference) => NestedAttrTypeFrame::Reference(reference),
            NestedAttrTypeFrame::Value(val) => NestedAttrTypeFrame::Value(val),
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
            NestedAttrType::Array(arr) => NestedAttrTypeFrame::Array(*arr),
            NestedAttrType::Null => NestedAttrTypeFrame::Null,
        }
    }
}

impl<'de> Deserialize<'de> for NestedAttrType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input: serde_json::Value = serde_json::Value::deserialize(deserializer)?;

        let expanded = NestedResult::expand_frames(input, |seed| match seed {
            serde_json::Value::String(text) => match text.parse::<RefValue>() {
                Ok(ref_value) => NestedResultFrame(Ok(NestedAttrTypeFrame::Reference(ref_value))),
                Err(_) => match text.parse::<AttributeType>() {
                    Ok(attribute_type) => NestedResultFrame(Ok(NestedAttrTypeFrame::Value(attribute_type))),
                    Err(_) => NestedResultFrame(Err(AttributeError::General(format!("Can't parse attribute type: {}", text)))),
                },
            },
            serde_json::Value::Array(arr) => NestedResultFrame(Ok(NestedAttrTypeFrame::Array(arr[0].clone()))),
            e => NestedResultFrame(Err(AttributeError::General(format!("Unexpected json value: {}", e.to_string())))),
        });
        match expanded.0 {
            Ok(el) => Ok(el),
            Err(er) => Err(er).map_err(serde::de::Error::custom),
        }
    }
}

#[cfg(test)]
mod tests {
    use said::derivation::{HashFunction, HashFunctionCode};

    use crate::ast::{AttributeType, NestedAttrType, RefValue};


    #[test]
    fn test_nested_array_attribute_type_serialization() {

        let arr = NestedAttrType::Array(Box::new(NestedAttrType::Array(Box::new(NestedAttrType::Value(AttributeType::Boolean)))));
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
        println!("{:?}", deser);
        assert!(deser.is_err());


        let serialized = r#"["refs:EEokfxxqwAM08iku7VHMaVFBaEGYVi2W-ctBKaTW6QdJ"]"#;
        let expected = NestedAttrType::Array(Box::new(NestedAttrType::Reference(RefValue::Said(
            said::derivation::HashFunction::from(said::derivation::HashFunctionCode::Blake3_256)
                .derive("fff".as_bytes()),
        ))));
        let deser: NestedAttrType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(expected, deser);
    }
}
