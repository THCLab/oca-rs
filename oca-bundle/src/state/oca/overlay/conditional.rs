use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;
use said::{sad::SAD, sad::SerializationFormats, derivation::HashFunctionCode};

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: String,
    capture_base: Option<said::SelfAddressingIdentifier>,
    pub attribute_conditions: BTreeMap<String, String>,
    pub attribute_dependencies: BTreeMap<String, Vec<String>>,
}

impl Overlay for ConditionalOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.capture_base
    }
    fn set_capture_base(&mut self, said: &said::SelfAddressingIdentifier) {
        self.capture_base = Some(said.clone());
    }
    fn said(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.said
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
            capture_base: None,
            said: None,
            overlay_type: "spec/overlays/conditional/1.0".to_string(),
            attribute_conditions: BTreeMap::new(),
            attribute_dependencies: BTreeMap::new(),
        })
    }
}
