use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardinalityOverlay {
    capture_base: String,
    #[serde(rename = "digest")]
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub attribute_cardinality: HashMap<String, String>,
}

impl Overlay for CardinalityOverlay {
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
        self.attribute_cardinality.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.cardinality.is_some() {
            self.attribute_cardinality.insert(
                attribute.name.clone(),
                attribute.cardinality.as_ref().unwrap().clone(),
            );
        }
    }
}
impl CardinalityOverlay {
    pub fn new() -> Box<CardinalityOverlay> {
        Box::new(CardinalityOverlay {
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/cardinality/1.0".to_string(),
            attribute_cardinality: HashMap::new(),
        })
    }
}
