use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub attribute_conditions: BTreeMap<String, String>,
    pub attribute_dependencies: BTreeMap<String, Vec<String>>,
}

impl Overlay for ConditionalOverlay {
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
        self.attribute_conditions.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.condition.is_some() {
            self.attribute_conditions.insert(
                attribute.name.clone(),
                attribute.condition.as_ref().unwrap().clone(),
            );
        }
        if attribute.dependencies.is_some() {
            self.attribute_dependencies.insert(
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
            attribute_conditions: BTreeMap::new(),
            attribute_dependencies: BTreeMap::new(),
        })
    }
}
