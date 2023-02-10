use crate::state::attribute::{Attribute, AttributeType};
use said::derivation::SelfAddressing;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CaptureBase {
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(rename = "digest")]
    pub said: String,
    pub classification: String,
    pub attributes: BTreeMap<String, String>,
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
            said: String::from("############################################"),
            classification: String::from(""),
            attributes: BTreeMap::new(),
            flagged_attributes: Vec::new(),
        }
    }

    pub fn add(&mut self, attribute: &Attribute) {
        let mut attr_type_str: String =
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
        }
        self.attributes
            .insert(attribute.name.clone(), attr_type_str);
        if attribute.is_flagged {
            self.flagged_attributes.push(attribute.name.clone());
        }
    }

    pub fn calculate_said(&self) -> String {
        let self_json = serde_json::to_string(&self).unwrap();

        format!(
            "{}",
            SelfAddressing::Blake3_256.derive(
                self_json
                    .replace(
                        self.said.as_str(),
                        "############################################"
                    )
                    .as_bytes()
            )
        )
    }

    pub fn sign(&mut self) {
        self.said = self.calculate_said();
    }
}
