use crate::state::attribute::Attribute;
use indexmap::IndexMap;
use oca_ast_semantics::ast::NestedAttrType;
use said::{
    derivation::HashFunctionCode,
    sad::{SerializationFormats, SAD},
};
use serde::{ser::SerializeMap, ser::SerializeSeq, Deserialize, Serialize, Serializer};

pub fn serialize_attributes<S>(
    attributes: &IndexMap<String, NestedAttrType>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use std::collections::BTreeMap;

    let mut ser = s.serialize_map(Some(attributes.len()))?;
    let sorted_attributes: BTreeMap<_, _> = attributes.iter().collect();
    for (k, v) in sorted_attributes {
        ser.serialize_entry(k, v)?;
    }
    ser.end()
}

pub fn serialize_flagged_attributes<S>(attributes: &[String], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ser = s.serialize_seq(Some(attributes.len()))?;

    let mut sorted_flagged_attributes = attributes.to_owned();
    sorted_flagged_attributes.sort();
    for attr in sorted_flagged_attributes {
        ser.serialize_element(&attr)?;
    }
    ser.end()
}

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct CaptureBase {
    #[said]
    #[serde(rename = "d")]
    pub said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(serialize_with = "serialize_attributes")]
    pub attributes: IndexMap<String, NestedAttrType>,
    pub classification: String,
    #[serde(serialize_with = "serialize_flagged_attributes")]
    pub flagged_attributes: Vec<String>,
}

impl Default for CaptureBase {
    fn default() -> Self {
        Self::new()
    }
}

impl CaptureBase {
    pub fn new() -> CaptureBase {
        CaptureBase {
            schema_type: String::from("spec/capture_base/1.0"),
            said: None,
            classification: String::from(""),
            attributes: IndexMap::new(),
            flagged_attributes: Vec::new(),
        }
    }

    pub fn set_classification(&mut self, classification: &str) {
        self.classification = classification.to_string();
    }

    pub fn add(&mut self, attribute: &Attribute) {
        /* let mut attr_type_str: AttributeType =
            serde_json::from_value(serde_json::to_value(attribute.attribute_type).unwrap())
                .unwrap();
        if let Some(AttributeType::Reference) = attribute.attribute_type {
            attr_type_str.push(':');
            attr_type_str.push_str(attribute.reference_sai.as_ref().unwrap_or(&"".to_string()));
        }
        if let Some(AttributeType::ArrayReference) = attribute.attribute_type {
            attr_type_str.pop();
            attr_type_str.push(':');
            attr_type_str.push_str(attribute.reference_sai.as_ref().unwrap_or(&"".to_string()));
            attr_type_str.push(']');
        }*/
        self.attributes.insert(
            attribute.name.clone(),
            attribute.attribute_type.clone().unwrap(),
        );
        if attribute.is_flagged {
            self.flagged_attributes.push(attribute.name.clone());
        }
    }

    pub fn fill_said(&mut self) {
        let code = HashFunctionCode::Blake3_256;
        let format = SerializationFormats::JSON;
        self.compute_digest(&code, &format);
    }

    pub fn sign(&mut self) {
        self.fill_said();
    }
}
