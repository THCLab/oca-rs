use super::{standard::Standard, oca::overlay::unit::{MeasurementSystem, MeasurementUnit}};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};
use wasm_bindgen::prelude::*;
use isolang::Language;

use crate::state::{encoding::Encoding, entry_codes::EntryCodes};

pub struct Attribute {
    pub name: String,
    pub attribute_type: Option<AttributeType>,
    pub is_flagged: bool,
    pub labels: Option<HashMap<Language, String>>,
    pub category_labels: Option<HashMap<Language, String>>,
    pub informations: Option<HashMap<Language, String>>,
    pub entry_codes: Option<EntryCodes>,
    pub mapping: Option<String>,
    pub encoding: Option<Encoding>,
    pub format: Option<String>,
    pub units: Option<HashMap<MeasurementSystem, MeasurementUnit>>,
    //pub entry_codes: Option<EntryCodes>,
    pub entry_codes_mapping: Option<Vec<String>>,
    pub reference_sai: Option<String>, // replace with SAI and move to RefAttribute
    pub condition: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub cardinality: Option<String>,
    pub conformance: Option<String>,
    pub standard: Option<Standard>,
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
            format: None,
            units: None,
            entry_codes: None,
            entry_codes_mapping: None,
            reference_sai: None, // TODO: replace with RefAttribute which consist only with reference to another object
            condition: None,
            dependencies: None,
            cardinality: None,
            conformance: None,
            standard: None,
        }
    }

    pub fn set_flagged(mut self) -> () {
        self.is_flagged = true;
    }

    pub fn set_attribute_type(mut self, attribute_type: AttributeType) -> () {
        self.attribute_type = Some(attribute_type);
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

    // pub fn add_cardinality(mut self, cardinality: String) -> AttributeBuilder {
    //     self.attribute.cardinality = Some(cardinality);
    //     self
    // }

    // pub fn add_conformance(mut self, conformance: String) -> AttributeBuilder {
    //     self.attribute.conformance = Some(conformance);
    //     self
    // }

    // pub fn add_encoding(mut self, encoding: Encoding) -> AttributeBuilder {
    //     self.attribute.encoding = Some(encoding);
    //     self
    // }

    // pub fn add_mapping(mut self, mapping: String) -> AttributeBuilder {
    //     self.attribute.mapping = Some(mapping);
    //     self
    // }

    // pub fn add_sai(mut self, sai: String) -> AttributeBuilder {
    //     self.attribute.sai = Some(sai);
    //     self
    // }

    // pub fn add_format(mut self, format: String) -> AttributeBuilder {
    //     self.attribute.format = Some(format);
    //     self
    // }

    // pub fn add_standard(mut self, standard: String) -> AttributeBuilder {
    //     self.attribute.standard = Some(Standard::new(standard));
    //     self
    // }

     // pub fn add_entry_codes(mut self, entry_codes: EntryCodes) -> AttributeBuilder {
    //     self.attribute.entry_codes = Some(entry_codes);
    //     self
    // }

    // pub fn add_entry_codes_mapping(mut self, mapping: Vec<String>) -> AttributeBuilder {
    //     self.attribute.entry_codes_mapping = Some(mapping);
    //     self
    // }

    // pub fn add_entries(mut self, entries: Entries) -> AttributeBuilder {
    //     match entries {
    //         Entries::Sai(lang_sai) => {
    //             for (lang, sai) in lang_sai.iter() {
    //                 match self.attribute.translations.get_mut(lang) {
    //                     Some(t) => {
    //                         t.add_entries_sai(sai.to_string());
    //                     }
    //                     None => {
    //                         let mut tr = AttributeTranslation::new();
    //                         tr.add_entries_sai(sai.to_string());
    //                         self.attribute.translations.insert(lang.clone(), tr);
    //                     }
    //                 }
    //             }
    //         }
    //         Entries::Object(entries_vec) => {
    //             for entry in entries_vec.iter() {
    //                 for (lang, en) in entry.translations.iter() {
    //                     match self.attribute.translations.get_mut(lang) {
    //                         Some(t) => {
    //                             t.add_entry(entry.id.clone(), en.clone());
    //                         }
    //                         None => {
    //                             let mut tr = AttributeTranslation::new();
    //                             tr.add_entry(entry.id.clone(), en.clone());
    //                             self.attribute.translations.insert(lang.clone(), tr);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Entries {
    Sai(HashMap<Language, String>),
    Object(Vec<Entry>),
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
    DateTime,
    #[serde(rename = "Array[DateTime]")]
    ArrayDateTime,
    Reference,
    #[serde(rename = "Array[Reference]")]
    ArrayReference,
}

impl ToString for AttributeType {
    fn to_string(&self) -> String {
        match self {
            AttributeType::Boolean => "Boolean".to_string(),
            AttributeType::ArrayBoolean => "Array[Boolean]".to_string(),
            AttributeType::Binary => "Binary".to_string(),
            AttributeType::ArrayBinary => "Array[Binary]".to_string(),
            AttributeType::Text => "Text".to_string(),
            AttributeType::ArrayText => "Array[Text]".to_string(),
            AttributeType::Numeric => "Numeric".to_string(),
            AttributeType::ArrayNumeric => "Array[Numeric]".to_string(),
            AttributeType::DateTime => "DateTime".to_string(),
            AttributeType::ArrayDateTime => "Array[DateTime]".to_string(),
            AttributeType::Reference => "Reference".to_string(),
            AttributeType::ArrayReference => "Array[Reference]".to_string(),
        }
    }
}

impl FromStr for AttributeType {
    type Err = String;

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
            _ => Err(format!("{} is not a valid AttributeType", s)),
        }
    }
}
