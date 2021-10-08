use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::state::{encoding::Encoding, language::Language};

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub attr_type: AttributeType,
    pub is_pii: bool,
    pub translations: HashMap<Language, AttributeTranslation>,
    pub encoding: Option<Encoding>,
    pub format: Option<String>,
    pub unit: Option<String>,
    pub entry_codes: Option<Vec<String>>,
}

impl Attribute {
    pub fn new(name: String, attr_type: AttributeType) -> Attribute {
        Attribute {
            name,
            attr_type,
            is_pii: false,
            translations: HashMap::new(),
            encoding: None,
            format: None,
            unit: None,
            entry_codes: None,
        }
    }

    pub fn set_pii(mut self) -> Attribute {
        self.is_pii = true;
        self
    }

    pub fn add_encoding(mut self, encoding: Encoding) -> Attribute {
        self.encoding = Some(encoding);
        self
    }

    pub fn add_format(mut self, format: String) -> Attribute {
        self.format = Some(format);
        self
    }

    pub fn add_unit(mut self, unit: String) -> Attribute {
        self.unit = Some(unit);
        self
    }

    pub fn add_label(mut self, labels: HashMap<Language, String>) -> Attribute {
        for (lang, label) in labels.iter() {
            match self.translations.get_mut(lang) {
                Some(t) => {
                    t.add_label(label.clone());
                }
                None => {
                    let mut tr = AttributeTranslation::new();
                    tr.add_label(label.clone());
                    self.translations.insert(*lang, tr);
                }
            }
        }
        self
    }

    pub fn add_entries(mut self, entries: Vec<Entry>) -> Attribute {
        let mut entry_codes = vec![];

        for entry in entries.iter() {
            entry_codes.push(entry.id.clone());

            for (lang, en) in entry.translations.iter() {
                match self.translations.get_mut(lang) {
                    Some(t) => {
                        t.add_entry(entry.id.clone(), en.clone());
                    }
                    None => {
                        let mut tr = AttributeTranslation::new();
                        tr.add_entry(entry.id.clone(), en.clone());
                        self.translations.insert(*lang, tr);
                    }
                }
            }
        }
        self.entry_codes = Some(entry_codes);

        self
    }

    pub fn add_information(mut self, information: HashMap<Language, String>) -> Attribute {
        for (lang, info) in information.iter() {
            match self.translations.get_mut(lang) {
                Some(t) => {
                    t.add_information(info.clone());
                }
                None => {
                    let mut tr = AttributeTranslation::new();
                    tr.add_information(info.clone());
                    self.translations.insert(*lang, tr);
                }
            }
        }
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    id: String,
    translations: HashMap<Language, String>,
}

impl Entry {
    pub fn new(id: String, translations: HashMap<Language, String>) -> Entry {
        Entry { id, translations }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeTranslation {
    pub label: Option<String>,
    pub entries: Option<HashMap<String, String>>,
    pub information: Option<String>,
}

impl Default for AttributeTranslation {
    fn default() -> Self {
        Self::new()
    }
}

impl AttributeTranslation {
    pub fn new() -> AttributeTranslation {
        AttributeTranslation {
            label: None,
            entries: None,
            information: None,
        }
    }

    pub fn add_label(&mut self, label: String) -> &mut AttributeTranslation {
        self.label = Some(label);
        self
    }

    pub fn add_entry(&mut self, id: String, tr: String) -> &mut AttributeTranslation {
        if self.entries.is_none() {
            self.entries = Some(HashMap::new());
        }
        if let Some(mut entries) = self.entries.clone() {
            entries.insert(id, tr);
            self.entries = Some(entries);
        }
        self
    }

    pub fn add_information(&mut self, information: String) -> &mut AttributeTranslation {
        self.information = Some(information);
        self
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AttributeType {
    Text,
    Number,
    Date,
    #[serde(rename = "Array[Text]")]
    ArrayText,
}
