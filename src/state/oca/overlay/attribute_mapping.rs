use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeMappingOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub attr_mappings: BTreeMap<String, String>,
}

impl Overlay for AttributeMappingOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        self.attr_mappings.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.mapping.is_some() {
            self.attr_mappings.insert(
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
            overlay_type: "spec/overlays/mapping/1.0".to_string(),
            attr_mappings: BTreeMap::new(),
        })
    }
}
