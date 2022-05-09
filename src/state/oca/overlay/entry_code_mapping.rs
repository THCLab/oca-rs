use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryCodeMappingOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    attr_mapping: BTreeMap<String, Vec<String>>,
}

impl Overlay for EntryCodeMappingOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        self.attr_mapping.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.entry_codes_mapping.is_some() {
            self.attr_mapping.insert(
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
            overlay_type: "spec/overlays/entry_code_mapping/1.0".to_string(),
            attr_mapping: BTreeMap::new(),
        })
    }
}
