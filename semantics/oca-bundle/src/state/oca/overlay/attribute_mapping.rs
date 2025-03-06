use crate::state::{attribute::Attribute, oca::Overlay};
use oca_ast_semantics::ast::OverlayType;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct AttributeMappingOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: OverlayType,
    capture_base: Option<said::SelfAddressingIdentifier>,
    pub attribute_mapping: BTreeMap<String, String>,
}

impl Overlay for AttributeMappingOverlay {
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
        self.attribute_mapping.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, _attribute: &Attribute) {
        // if attribute.mapping.is_some() {
        //     self.attribute_mapping.insert(
        //         attribute.name.clone(),
        //         attribute.mapping.as_ref().unwrap().clone(),
        //     );
        // }
    }
}
impl AttributeMappingOverlay {
    pub fn new() -> Box<Self> {
        let overlay_version = "1.1".to_string();
        Box::new(Self {
            capture_base: None,
            said: None,
            overlay_type: OverlayType::AttributeMapping(overlay_version),
            attribute_mapping: BTreeMap::new(),
        })
    }
}
