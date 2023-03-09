use crate::state::{
    attribute::Attribute, entries::EntriesElement, oca::Overlay,
};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
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

impl Serialize for EntryOverlay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use std::collections::BTreeMap;

        let mut state = serializer.serialize_struct("EntryOverlay", 4)?;
        state.serialize_field("said", &self.said)?;
        state.serialize_field("language", &self.language)?;
        state.serialize_field("type", &self.overlay_type)?;
        state.serialize_field("capture_base", &self.capture_base)?;
        let sorted_attribute_entries: BTreeMap<_, _> = self.attribute_entries.iter().collect();
        state.serialize_field("attribute_entries", &sorted_attribute_entries)?;
        state.end()
    }
}

#[derive(Deserialize, Debug, Clone)]
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
