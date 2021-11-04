use crate::state::attribute::{Attribute, AttributeType};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CaptureBase {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub classification: String,
    pub attributes: BTreeMap<String, AttributeType>,
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
        self.attributes
            .insert(attribute.name.clone(), attribute.attr_type);
        if attribute.is_pii {
            self.pii.push(attribute.name.clone());
        }
    }
}
