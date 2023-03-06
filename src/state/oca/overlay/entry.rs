use crate::state::{
    attribute::Attribute, entries::EntriesElement, oca::Overlay,
};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use isolang::Language;

pub trait Entries {
    fn set_entry(&mut self, l: Language, entry: EntriesElement);
}

impl Entries for Attribute {
    fn set_entry(&mut self, l: Language, entry: EntriesElement) {
        if let Some(entries) = &mut self.entries {
            entries.insert(l, entry);
        } else {
            let mut entries = HashMap::new();
            entries.insert(l, entry);
            self.entries = Some(entries);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryOverlay {
    capture_base: String,
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    pub attribute_entries: HashMap<String, EntriesElement>,
}

impl Overlay for EntryOverlay {
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
    fn language(&self) -> Option<&Language> {
        Some(&self.language)
    }
    fn attributes(&self) -> Vec<&String> {
        self.attribute_entries.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(entries) = &attribute.entries {
            if let Some(tr) = entries.get(&self.language) {
                self.attribute_entries
                    .insert(attribute.name.clone(), tr.clone());
            }
        }
    }
}
impl EntryOverlay {
    pub fn new(lang: Language) -> EntryOverlay {
        EntryOverlay {
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/entry/1.0".to_string(),
            language: lang,
            attribute_entries: HashMap::new(),
        }
    }
}
