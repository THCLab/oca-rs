use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubsetOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub attributes: Vec<String>,
}

impl Overlay for SubsetOverlay {
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
        self.attributes.iter().collect()
    }

    fn add(&mut self, attribute: &Attribute) {
        self.attributes.push(attribute.name.clone());
    }
}
impl SubsetOverlay {
    pub fn new() -> Box<SubsetOverlay> {
        Box::new(SubsetOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overlays/subset/1.0".to_string(),
            attributes: vec![],
        })
    }
}
