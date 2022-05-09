use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use wasm_bindgen::prelude::*;

use crate::state::{
    encoding::Encoding, entries::EntriesElement, entry_codes::EntryCodes, language::Language,
};

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub attr_type: AttributeType,
    pub is_pii: bool,
    pub translations: HashMap<Language, AttributeTranslation>,
    pub mapping: Option<String>,
    pub encoding: Option<Encoding>,
    pub format: Option<String>,
    pub metric_system: Option<String>,
    pub unit: Option<String>,
    pub entry_codes: Option<EntryCodes>,
    pub sai: Option<String>,
    pub condition: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub cardinality: Option<String>,
    pub conformance: Option<String>,
}

pub struct AttributeBuilder {
    pub attribute: Attribute,
}

impl AttributeBuilder {
    pub fn new(name: String, attr_type: AttributeType) -> AttributeBuilder {
        AttributeBuilder {
            attribute: Attribute {
                name,
                attr_type,
                is_pii: false,
                translations: HashMap::new(),
                mapping: None,
                encoding: None,
                format: None,
                metric_system: None,
                unit: None,
                entry_codes: None,
                sai: None,
                condition: None,
                dependencies: None,
                cardinality: None,
                conformance: None,
            },
        }
    }

    pub fn set_pii(mut self) -> AttributeBuilder {
        self.attribute.is_pii = true;
        self
    }

    pub fn add_condition(
        mut self,
        condition: String,
        dependencies: Vec<String>,
    ) -> AttributeBuilder {
        self.attribute.condition = Some(condition);
        self.attribute.dependencies = Some(dependencies);
        self
    }

    pub fn add_cardinality(
        mut self,
        cardinality: String,
    ) -> AttributeBuilder {
        self.attribute.cardinality = Some(cardinality);
        self
    }

    pub fn add_conformance(
        mut self,
        conformance: String,
    ) -> AttributeBuilder {
        self.attribute.conformance = Some(conformance);
        self
    }

    pub fn add_encoding(mut self, encoding: Encoding) -> AttributeBuilder {
        self.attribute.encoding = Some(encoding);
        self
    }

    pub fn add_mapping(
        mut self,
        mapping: String,
    ) -> AttributeBuilder {
        self.attribute.mapping = Some(mapping);
        self
    }

    pub fn add_sai(mut self, sai: String) -> AttributeBuilder {
        self.attribute.sai = Some(sai);
        self
    }

    pub fn add_format(mut self, format: String) -> AttributeBuilder {
        self.attribute.format = Some(format);
        self
    }

    pub fn add_unit(mut self, metric_system: String, unit: String) -> AttributeBuilder {
        self.attribute.metric_system = Some(metric_system);
        self.attribute.unit = Some(unit);
        self
    }

    pub fn add_label(mut self, labels: HashMap<Language, String>) -> AttributeBuilder {
        for (lang, label) in labels.iter() {
            match self.attribute.translations.get_mut(lang) {
                Some(t) => {
                    t.add_label(label.clone());
                }
                None => {
                    let mut tr = AttributeTranslation::new();
                    tr.add_label(label.clone());
                    self.attribute.translations.insert(lang.clone(), tr);
                }
            }
        }
        self
    }

    pub fn add_entry_codes(mut self, entry_codes: EntryCodes) -> AttributeBuilder {
        self.attribute.entry_codes = Some(entry_codes);
        self
    }

    pub fn add_entries(mut self, entries: Entries) -> AttributeBuilder {
        match entries {
            Entries::Sai(lang_sai) => {
                for (lang, sai) in lang_sai.iter() {
                    match self.attribute.translations.get_mut(lang) {
                        Some(t) => {
                            t.add_entries_sai(sai.to_string());
                        }
                        None => {
                            let mut tr = AttributeTranslation::new();
                            tr.add_entries_sai(sai.to_string());
                            self.attribute.translations.insert(lang.clone(), tr);
                        }
                    }
                }
            }
            Entries::Object(entries_vec) => {
                for entry in entries_vec.iter() {
                    for (lang, en) in entry.translations.iter() {
                        match self.attribute.translations.get_mut(lang) {
                            Some(t) => {
                                t.add_entry(entry.id.clone(), en.clone());
                            }
                            None => {
                                let mut tr = AttributeTranslation::new();
                                tr.add_entry(entry.id.clone(), en.clone());
                                self.attribute.translations.insert(lang.clone(), tr);
                            }
                        }
                    }
                }
            }
        }
        self
    }

    pub fn add_information(mut self, information: HashMap<Language, String>) -> AttributeBuilder {
        for (lang, info) in information.iter() {
            match self.attribute.translations.get_mut(lang) {
                Some(t) => {
                    t.add_information(info.clone());
                }
                None => {
                    let mut tr = AttributeTranslation::new();
                    tr.add_information(info.clone());
                    self.attribute.translations.insert(lang.clone(), tr);
                }
            }
        }
        self
    }

    pub fn build(self) -> Attribute {
        self.attribute
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub translations: HashMap<Language, String>,
}

impl Entry {
    pub fn new(id: String, translations: HashMap<Language, String>) -> Entry {
        Entry { id, translations }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Entries {
    Sai(HashMap<Language, String>),
    Object(Vec<Entry>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeTranslation {
    pub label: Option<String>,
    pub entries: Option<EntriesElement>,
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

    pub fn add_entries_sai(&mut self, sai: String) -> &mut AttributeTranslation {
        self.entries = Some(EntriesElement::Sai(sai));
        self
    }

    pub fn add_entry(&mut self, id: String, tr: String) -> &mut AttributeTranslation {
        if self.entries.is_none() {
            self.entries = Some(EntriesElement::Object(BTreeMap::new()));
        }
        if let Some(EntriesElement::Object(mut entries)) = self.entries.clone() {
            entries.insert(id, tr);
            self.entries = Some(EntriesElement::Object(entries));
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
    Boolean,
    #[serde(rename = "Array[Boolean]")]
    ArrayBoolean,
    Binary,
    #[serde(rename = "Array[Binary]")]
    ArrayBinary,
    Text,
    #[serde(rename = "Array[Text]")]
    ArrayText,
    Numeric,
    #[serde(rename = "Array[Numeric]")]
    ArrayNumeric,
    Date,
    #[serde(rename = "Array[Date]")]
    ArrayDate,
    #[serde(rename = "SAI")]
    Sai,
    #[serde(rename = "Array[SAI]")]
    ArraySai,
}
