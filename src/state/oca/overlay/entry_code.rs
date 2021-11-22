use crate::state::{attribute::Attribute, oca::Overlay, entry_codes::EntryCodes};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryCodeOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    attr_entry_codes: BTreeMap<String, EntryCodes>,
}

impl Overlay for EntryCodeOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        self.attr_entry_codes.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.entry_codes.is_some() {
            self.attr_entry_codes.insert(
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
            overlay_type: "spec/overlays/entry_code/1.0".to_string(),
            attr_entry_codes: BTreeMap::new(),
        })
    }
}
