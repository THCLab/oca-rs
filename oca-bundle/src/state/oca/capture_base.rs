use crate::state::attribute::{Attribute, AttributeType};
use said::derivation::SelfAddressing;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use std::collections::HashMap;

impl Serialize for CaptureBase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use std::collections::BTreeMap;

        let mut state = serializer.serialize_struct("CaptureBase", 5)?;
        state.serialize_field("said", &self.said)?;
        state.serialize_field("type", &self.schema_type)?;
        state.serialize_field("classification", &self.classification)?;
        let sorted_attributes: BTreeMap<_, _> = self.attributes.iter().collect();
        state.serialize_field("attributes", &sorted_attributes)?;
        let mut sorted_flagged_attributes = self.flagged_attributes.clone();
        sorted_flagged_attributes.sort();
        state.serialize_field("flagged_attributes", &sorted_flagged_attributes)?;
        state.end()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CaptureBase {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub said: String,
    pub classification: String,
    pub attributes: HashMap<String, String>,
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
            attributes: HashMap::new(),
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
