use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeMappingOverlay {
    capture_base: String,
    #[serde(rename = "digest")]
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub attribute_mapping: BTreeMap<String, String>,
}

impl Overlay for AttributeMappingOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn said(&self) -> &String {
        &self.said
    }
    fn said_mut(&mut self) -> &mut String {
        &mut self.said
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        self.attribute_mapping.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.mapping.is_some() {
            self.attribute_mapping.insert(
                attribute.name.clone(),
                attribute.mapping.as_ref().unwrap().clone(),
            );
        }
    }
}
impl AttributeMappingOverlay {
    pub fn new() -> Box<AttributeMappingOverlay> {
        Box::new(AttributeMappingOverlay {
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/mapping/1.0".to_string(),
            attribute_mapping: BTreeMap::new(),
        })
    }
}
