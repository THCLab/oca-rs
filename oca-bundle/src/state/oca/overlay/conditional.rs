use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalOverlay {
    capture_base: String,
    #[serde(rename = "digest")]
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub attribute_conditions: BTreeMap<String, String>,
    pub attribute_dependencies: BTreeMap<String, Vec<String>>,
}

impl Overlay for ConditionalOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &String {
        &self.capture_base
    }
    fn capture_base_mut(&mut self) -> &mut String {
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
            said: String::from("############################################"),
            overlay_type: "spec/overlays/conditional/1.0".to_string(),
            attribute_conditions: BTreeMap::new(),
            attribute_dependencies: BTreeMap::new(),
        })
    }
}
