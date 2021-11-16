use crate::state::{attribute::Attribute, language::Language, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    attr_entries: BTreeMap<String, BTreeMap<String, String>>,
}

impl Overlay for EntryOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn language(&self) -> Option<&Language> {
        Some(&self.language)
    }
    fn attributes(&self) -> Vec<&String> {
        self.attr_entries.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(tr) = attribute.translations.get(&self.language) {
            if let Some(entries) = &tr.entries {
                self.attr_entries
                    .insert(attribute.name.clone(), entries.clone());
            }
        }
    }
}
impl EntryOverlay {
    pub fn new(lang: Language) -> Box<EntryOverlay> {
        Box::new(EntryOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overlays/entry/1.0".to_string(),
            language: lang,
            attr_entries: BTreeMap::new(),
        })
    }
}
