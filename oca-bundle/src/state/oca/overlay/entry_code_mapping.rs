use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryCodeMappingOverlay {
    capture_base: String,
    #[serde(rename = "digest")]
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub attribute_entry_codes_mapping: BTreeMap<String, Vec<String>>,
}

impl Overlay for EntryCodeMappingOverlay {
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
    pub fn new() -> Box<EntryCodeMappingOverlay> {
        Box::new(EntryCodeMappingOverlay {
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/entry_code_mapping/1.0".to_string(),
            attribute_entry_codes_mapping: BTreeMap::new(),
        })
    }
}
