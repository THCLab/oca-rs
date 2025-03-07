use crate::state::{attribute::Attribute, oca::Overlay};
use oca_ast_semantics::ast::OverlayType;
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct EntryCodeMappingOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    capture_base: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: OverlayType,
    pub attribute_entry_codes_mapping: BTreeMap<String, Vec<String>>,
}

impl Overlay for EntryCodeMappingOverlay {
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
        self.attribute_entry_codes_mapping
            .keys()
            .collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.entry_codes_mapping.is_some() {
            self.attribute_entry_codes_mapping.insert(
                attribute.name.clone(),
                attribute.entry_codes_mapping.as_ref().unwrap().clone(),
            );
        }
    }
}
impl EntryCodeMappingOverlay {
    pub fn new() -> Box<Self> {
        let overlay_version = "1.1".to_string();
        Box::new(Self {
            capture_base: None,
            said: None,
            overlay_type: OverlayType::EntryCodeMapping(overlay_version),
            attribute_entry_codes_mapping: BTreeMap::new(),
        })
    }
}
