use super::{
    oca::overlay::unit::{MeasurementSystem, MeasurementUnit},
    standard::Standard,
};
use isolang::Language;
use oca_ast::ast::NestedAttrType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub use oca_ast::ast::AttributeType;

use crate::state::{encoding::Encoding, entry_codes::EntryCodes, entries::EntriesElement};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: Option<NestedAttrType>,
    pub is_flagged: bool,
    pub labels: Option<HashMap<Language, String>>,
    pub category_labels: Option<HashMap<Language, String>>,
    pub informations: Option<HashMap<Language, String>>,
    pub entry_codes: Option<EntryCodes>,
    pub entries: Option<HashMap<Language, EntriesElement>>,
    pub mapping: Option<String>,
    pub encoding: Option<Encoding>,
    #[cfg(feature = "format_overlay")]
    pub format: Option<String>,
    pub units: Option<HashMap<MeasurementSystem, MeasurementUnit>>,
    pub entry_codes_mapping: Option<Vec<String>>,
    pub condition: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub cardinality: Option<String>,
    pub conformance: Option<String>,
    pub standards: Option<Vec<Standard>>,
}

impl Default for Attribute {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

impl Attribute {
    pub fn new(name: String) -> Attribute {
        Attribute {
            name,
            labels: None,
            informations: None,
            category_labels: None,
            attribute_type: None,
            is_flagged: false,
            mapping: None,
            encoding: None,
            #[cfg(feature = "format_overlay")]
            format: None,
            units: None,
            entry_codes: None,
            entries: None,
            entry_codes_mapping: None,
            condition: None,
            dependencies: None,
            cardinality: None,
            conformance: None,
            standards: None,
        }
    }

    pub fn set_flagged(&mut self) {
        self.is_flagged = true;
    }

    pub fn set_attribute_type(&mut self, attribute_type: NestedAttrType) {
        self.attribute_type = Some(attribute_type);
    }


    // Merge assumption is that if `other` is not None then it would overwrite `self` or would be concatenated with `self`
    pub fn merge(&mut self, other: &Attribute) {
        if self.name != other.name {
            panic!("Cannot merge attributes with different names");
        } else {
            if other.attribute_type.is_some() {
                self.attribute_type = other.attribute_type.clone();
            }

            self.merge_labels(other);
            self.merge_information(other);
            self.merge_category_labels(other);

            if other.mapping.is_some() {
                self.mapping = other.mapping.clone();
            }

            if other.encoding.is_some() {
                self.encoding = other.encoding;
            }

            #[cfg(feature = "format_overlay")]
            if other.format.is_some() {
                self.format = other.format.clone();
            }

            if self.units.is_none() {
                self.units = other.units.clone();
            }

            if self.entry_codes.is_none() {
                self.entry_codes = other.entry_codes.clone();
            }

            self.merge_entries(other);

            if self.entry_codes_mapping.is_none() {
                self.entry_codes_mapping = other.entry_codes_mapping.clone();
            }

            if other.condition.is_some() {
                self.condition = other.condition.clone();

                if other.dependencies.is_some() {
                    self.dependencies = other.dependencies.clone();
                }
            }

            if other.cardinality.is_some() {
                self.cardinality = other.cardinality.clone();
            }

            if other.conformance.is_some() {
                self.conformance = other.conformance.clone();
            }

            if other.standards.is_some() {
                self.standards = other.standards.clone();
            }

        }
    }

    fn merge_entries(&mut self, other: &Attribute) {
        if self.entries.is_none() {
            self.entries = other.entries.clone();
        } else if let Some(entries) = &other.entries {
            for (lang, entry) in entries {
                self.entries
                    .as_mut()
                    .unwrap()
                    .insert(*lang, entry.clone());
            }
        }
    }

    fn merge_category_labels(&mut self, other: &Attribute) {
        if self.category_labels.is_none() {
            self.category_labels = other.category_labels.clone();
        } else if let Some(category_labels) = &other.category_labels {
            for (lang, category_label) in category_labels {
                self.category_labels
                    .as_mut()
                    .unwrap()
                    .insert(*lang, category_label.clone());
            }
        }
    }
    fn merge_information(&mut self, other: &Attribute) {
        if self.informations.is_none() {
            self.informations = other.informations.clone();
        } else if let Some(informations) = &other.informations {
            for (lang, information) in informations {
                self.informations
                    .as_mut()
                    .unwrap()
                    .insert(*lang, information.clone());
            }
        }
    }
    fn merge_labels(&mut self, other: &Attribute) {
        if self.labels.is_none() {
            self.labels = other.labels.clone();
        } else if let Some(labels) = &other.labels {
            for (lang, label) in labels {
                self.labels
                    .as_mut()
                    .unwrap()
                    .insert(*lang, label.clone());
            }
        }
    }

    // pub fn add_mapping(mut self, mapping: String) -> AttributeBuilder {
    //     self.attribute.mapping = Some(mapping);
    //     self
    // }

    // pub fn add_entry_codes_mapping(mut self, mapping: Vec<String>) -> AttributeBuilder {
    //     self.attribute.entry_codes_mapping = Some(mapping);
    //     self
    // }
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

/*
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Entries {
    Sai(HashMap<Language, String>),
    Object(Vec<Entry>),
}
*/
