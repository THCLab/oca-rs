use super::{
    oca::overlay::unit::{MeasurementSystem, MeasurementUnit},
    standard::Standard,
};
use isolang::Language;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use wasm_bindgen::prelude::*;

use crate::state::{encoding::Encoding, entry_codes::EntryCodes, entries::EntriesElement};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub attribute_type: Option<AttributeType>,
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
    pub reference_sai: Option<String>, // replace with SAI and move to RefAttribute
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
            reference_sai: None, // TODO: replace with RefAttribute which consist only with reference to another object
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

    pub fn set_attribute_type(&mut self, attribute_type: AttributeType) {
        self.attribute_type = Some(attribute_type);
    }

    pub fn set_sai(&mut self, sai: String) {
        self.reference_sai = Some(sai);
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
                self.encoding = other.encoding.clone();
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

            if self.entries.is_none() {
                self.entries = other.entries.clone();
            }

            if self.entry_codes_mapping.is_none() {
                self.entry_codes_mapping = other.entry_codes_mapping.clone();
            }

            if other.reference_sai.is_some() {
                self.reference_sai = other.reference_sai.clone();
            }

            if other.condition.is_some() {
                self.condition = other.condition.clone();
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

    fn merge_category_labels(&mut self, other: &Attribute) -> () {
        if self.category_labels.is_none() {
            self.category_labels = other.category_labels.clone();
        } else {
            if let Some(category_labels) = &other.category_labels {
                for (lang, category_label) in category_labels {
                    self.category_labels
                        .as_mut()
                        .unwrap()
                        .insert(lang.clone(), category_label.clone());
                }
            }
        }
    }
    fn merge_information(&mut self, other: &Attribute) -> () {
        if self.informations.is_none() {
            self.informations = other.informations.clone();
        } else {
            if let Some(informations) = &other.informations {
                for (lang, information) in informations {
                    self.informations
                        .as_mut()
                        .unwrap()
                        .insert(lang.clone(), information.clone());
                }
            }
        }
    }
    fn merge_labels(&mut self, other: &Attribute) -> () {
        if self.labels.is_none() {
            self.labels = other.labels.clone();
        } else {
            if let Some(labels) = &other.labels {
                for (lang, label) in labels {
                    self.labels
                        .as_mut()
                        .unwrap()
                        .insert(lang.clone(), label.clone());
                }
            }
        }
    }
    // pub fn add_condition(
    //     mut self,
    //     condition: String,
    //     dependencies: Vec<String>,
    // ) -> AttributeBuilder {
    //     self.attribute.condition = Some(condition);
    //     self.attribute.dependencies = Some(dependencies);
    //     self
    // }

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
    DateTime,
    #[serde(rename = "Array[DateTime]")]
    ArrayDateTime,
    Reference,
    #[serde(rename = "Array[Reference]")]
    ArrayReference,
}

impl FromStr for AttributeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Boolean" => Ok(AttributeType::Boolean),
            "Array[Boolean]" => Ok(AttributeType::ArrayBoolean),
            "Binary" => Ok(AttributeType::Binary),
            "Array[Binary]" => Ok(AttributeType::ArrayBinary),
            "Text" => Ok(AttributeType::Text),
            "Array[Text]" => Ok(AttributeType::ArrayText),
            "Numeric" => Ok(AttributeType::Numeric),
            "Array[Numeric]" => Ok(AttributeType::ArrayNumeric),
            "DateTime" => Ok(AttributeType::DateTime),
            "Array[DateTime]" => Ok(AttributeType::ArrayDateTime),
            "Reference" => Ok(AttributeType::Reference),
            "Array[Reference]" => Ok(AttributeType::ArrayReference),
            _ => Err(()),
        }
    }
}
