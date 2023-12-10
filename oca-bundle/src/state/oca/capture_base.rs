use crate::state::attribute::Attribute;
use oca_ast::ast::NestedAttrType;
use said::{sad::SAD, sad::SerializationFormats};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap, ser::SerializeSeq};
use std::collections::HashMap;

pub fn serialize_attributes<S>(attributes: &HashMap<String, NestedAttrType>, s: S) -> Result<S::Ok, S::Error>
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


pub fn serialize_flagged_attributes<S>(attributes: &Vec<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ser = s.serialize_seq(Some(attributes.len()))?;

    let mut sorted_flagged_attributes = attributes.clone();
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
    pub classification: String,
    #[serde(serialize_with = "serialize_attributes")]
    /// TODO do we need here indexmap?
    pub attributes: HashMap<String, NestedAttrType>,
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
            attributes: HashMap::new(),
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
        self.attributes.insert(attribute.name.clone(), attribute.attribute_type.clone().unwrap());
        if attribute.is_flagged {
            self.flagged_attributes.push(attribute.name.clone());
        }
    }

    pub fn fill_said(&mut self) {
        self.compute_digest(); //HashFunctionCode::Blake3_256, SerializationFormats::JSON);
    }

    pub fn sign(&mut self) {
        self.fill_said();
    }
}
