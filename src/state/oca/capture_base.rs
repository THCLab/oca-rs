use crate::state::attribute::{Attribute, AttributeType};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CaptureBase {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub classification: String,
    pub attributes: BTreeMap<String, String>,
    pub pii: Vec<String>,
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
            classification: String::from(""),
            attributes: BTreeMap::new(),
            pii: Vec::new(),
        }
    }

    pub fn add(&mut self, attribute: &Attribute) {
        let mut attr_type_str: String =
            serde_json::from_value(serde_json::to_value(&attribute.attr_type).unwrap()).unwrap();
        if let AttributeType::Sai = attribute.attr_type {
            attr_type_str.push(':');
            attr_type_str.push_str(attribute.sai.as_ref().unwrap_or(&"".to_string()));
        }
        self.attributes
            .insert(attribute.name.clone(), attr_type_str);
        if attribute.is_pii {
            self.pii.push(attribute.name.clone());
        }
    }
}
