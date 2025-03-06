use crate::state::{attribute::Attribute, oca::Overlay};
use oca_ast_semantics::ast::OverlayType;
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct SubsetOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    capture_base: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: OverlayType,
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
        let overlay_version = "1.1".to_string();
        Box::new(SubsetOverlay {
            capture_base: None,
            said: None,
            overlay_type: OverlayType::Subset(overlay_version),
            attributes: vec![],
        })
    }
}
