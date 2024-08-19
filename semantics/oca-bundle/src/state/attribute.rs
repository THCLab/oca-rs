use super::standard::Standard;
use isolang::Language;
pub use oca_ast_semantics::ast::AttributeType;
use oca_ast_semantics::ast::NestedAttrType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::state::{encoding::Encoding, entries::EntriesElement, entry_codes::EntryCodes};
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
    pub unit: Option<String>,
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
            unit: None,
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
                self.attribute_type.clone_from(&other.attribute_type);
            }

            self.merge_labels(other);
            self.merge_information(other);
            self.merge_category_labels(other);

            if other.mapping.is_some() {
                self.mapping.clone_from(&other.mapping);
            }

            if other.encoding.is_some() {
                self.encoding.clone_from(&other.encoding);
            }

            #[cfg(feature = "format_overlay")]
            if other.format.is_some() {
                self.format.clone_from(&other.format);
            }

            if self.unit.is_none() {
                self.unit.clone_from(&other.unit);
            }

            if self.entry_codes.is_none() {
                self.entry_codes.clone_from(&other.entry_codes);
            }

            self.merge_entries(other);

            if self.entry_codes_mapping.is_none() {
                self.entry_codes_mapping.clone_from(&other.entry_codes_mapping);
            }

            if other.condition.is_some() {
                self.condition.clone_from(&other.condition);

                if other.dependencies.is_some() {
                    self.dependencies.clone_from(&other.dependencies);
                }
            }

            if other.cardinality.is_some() {
                self.cardinality.clone_from(&other.cardinality);
            }

            if other.conformance.is_some() {
                self.conformance.clone_from(&other.conformance);
            }

            if other.standards.is_some() {
                self.standards.clone_from(&other.standards);
            }
        }
    }

    fn merge_entries(&mut self, other: &Attribute) {
        if self.entries.is_none() {
            self.entries.clone_from(&other.entries);
        } else if let Some(entries) = &other.entries {
            for (lang, entry) in entries {
                self.entries.as_mut().unwrap().insert(*lang, entry.clone());
            }
        }
    }

    fn merge_category_labels(&mut self, other: &Attribute) {
        if self.category_labels.is_none() {
            self.category_labels.clone_from(&other.category_labels);
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
            self.informations.clone_from(&other.informations);
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
            self.labels.clone_from(&other.labels)
        } else if let Some(labels) = &other.labels {
            for (lang, label) in labels {
                self.labels.as_mut().unwrap().insert(*lang, label.clone());
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
