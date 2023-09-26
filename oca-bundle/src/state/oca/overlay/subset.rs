use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;
use said::{sad::SAD, sad::SerializationFormats};
use oca_ast::ast::OverlayType;

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct SubsetOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: OverlayType,
    capture_base: Option<said::SelfAddressingIdentifier>,
    pub attributes: Vec<String>,
}

impl Overlay for SubsetOverlay {
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
    fn overlay_type(&self) -> &OverlayType {
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
            capture_base: None,
            said: None,
            overlay_type: OverlayType::Subset,
            attributes: vec![],
        })
    }
}
