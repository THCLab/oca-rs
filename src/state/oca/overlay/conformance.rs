use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConformanceOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    attr_conformance: BTreeMap<String, String>,
}

impl Overlay for ConformanceOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        self.attr_conformance.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.conformance.is_some() {
            self.attr_conformance.insert(
                attribute.name.clone(),
                attribute.conformance.as_ref().unwrap().clone(),
            );
        }
    }
}
impl ConformanceOverlay {
    pub fn new() -> Box<ConformanceOverlay> {
        Box::new(ConformanceOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overlays/conformance/1.0".to_string(),
            attr_conformance: BTreeMap::new(),
        })
    }
}
