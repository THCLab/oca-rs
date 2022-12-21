use crate::state::{attribute::Attribute, entry_codes::EntryCodes, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryCodeOverlay {
    capture_base: String,
    #[serde(rename = "digest")]
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub attribute_entry_codes: BTreeMap<String, EntryCodes>,
}

impl Overlay for EntryCodeOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&mut self) -> &mut String {
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
        self.attribute_entry_codes.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.entry_codes.is_some() {
            self.attribute_entry_codes.insert(
                attribute.name.clone(),
                attribute.entry_codes.as_ref().unwrap().clone(),
            );
        }
    }
}
impl EntryCodeOverlay {
    pub fn new() -> Box<EntryCodeOverlay> {
        Box::new(EntryCodeOverlay {
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/entry_code/1.0".to_string(),
            attribute_entry_codes: BTreeMap::new(),
        })
    }
}
