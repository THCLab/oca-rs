use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    attr_conditions: BTreeMap<String, String>,
    attr_dependencies: BTreeMap<String, Vec<String>>,
}

impl Overlay for ConditionalOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        self.attr_conditions.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.condition.is_some() {
            self.attr_conditions.insert(
                attribute.name.clone(),
                attribute.condition.as_ref().unwrap().clone(),
            );
        }
        if attribute.dependencies.is_some() {
            self.attr_dependencies.insert(
                attribute.name.clone(),
                attribute.dependencies.as_ref().unwrap().clone(),
            );
        }
    }
}
impl ConditionalOverlay {
    pub fn new() -> Box<ConditionalOverlay> {
        Box::new(ConditionalOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overlays/conditional/1.0".to_string(),
            attr_conditions: BTreeMap::new(),
            attr_dependencies: BTreeMap::new(),
        })
    }
}