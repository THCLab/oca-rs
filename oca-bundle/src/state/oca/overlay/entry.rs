use crate::state::{
    attribute::Attribute, entries::EntriesElement, oca::Overlay,
};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};
use std::any::Any;
use std::collections::HashMap;
use isolang::Language;
use said::{sad::SAD, sad::SerializationFormats, derivation::HashFunctionCode};

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

pub fn serialize_attributes<S>(attributes: &HashMap<String, EntriesElement>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use std::collections::BTreeMap;

    let mut ser = s.serialize_map(Some(attributes.len()))?;
    let sorted_attributes: BTreeMap<_, _> = attributes.iter().collect();
    for (k, v) in sorted_attributes {
        ser.serialize_entry(k, v)?;
    }
    ser.end()
}

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct EntryOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    language: Language,
    #[serde(rename = "type")]
    overlay_type: String,
    capture_base: Option<said::SelfAddressingIdentifier>,
    #[serde(serialize_with = "serialize_attributes")]
    pub attribute_entries: HashMap<String, EntriesElement>,
}

impl Overlay for EntryOverlay {
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
            capture_base: None,
            said: None,
            overlay_type: "spec/overlays/entry/1.0".to_string(),
            language: lang,
            attribute_entries: HashMap::new(),
        }
    }
}
