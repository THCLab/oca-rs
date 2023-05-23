use crate::state::{attribute::Attribute, oca::Overlay};
use isolang::Language;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap, ser::SerializeSeq};
use std::any::Any;
use std::collections::HashMap;
use said::{sad::SAD, sad::SerializationFormats, derivation::HashFunctionCode};

pub trait Labels {
    fn set_label(&mut self, l: Language, label: String) -> ();
    fn add_category_label(&mut self, l: Language, label: String) -> ();
}

impl Labels for Attribute {
    fn set_label(&mut self, l: Language, label: String) -> () {
        match self.labels {
            Some(ref mut labels) => {
                labels.insert(l, label);
            }
            None => {
                let mut labels = HashMap::new();
                labels.insert(l, label);
                self.labels = Some(labels);
            }
        }
    }
    fn add_category_label(&mut self, l: Language, label: String) -> () {
        match self.category_labels {
            Some(ref mut category_labels) => {
                category_labels.insert(l, label);
            }
            None => {
                let mut category_labels = HashMap::new();
                category_labels.insert(l, label);
                self.category_labels = Some(category_labels);
            }
        }
    }
}

pub fn serialize_labels<S>(attributes: &HashMap<String, String>, s: S) -> Result<S::Ok, S::Error>
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

pub fn serialize_categories<S>(attributes: &Vec<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ser = s.serialize_seq(Some(attributes.len()))?;

    let mut sorted_flagged_attributes = attributes.clone();
    sorted_flagged_attributes.sort();
    for attr in sorted_flagged_attributes {
        ser.serialize_element(&attr)?;
    }
    ser.end()
}

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct LabelOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    language: Language,
    #[serde(rename = "type")]
    overlay_type: String,
    capture_base: Option<said::SelfAddressingIdentifier>,
    #[serde(serialize_with = "serialize_labels")]
    pub attribute_labels: HashMap<String, String>,
    #[serde(serialize_with = "serialize_categories")]
    pub attribute_categories: Vec<String>, // TODO find out if we need duplicated structure to hold keys if we have hashmap with those keys
    #[serde(serialize_with = "serialize_labels")]
    pub category_labels: HashMap<String, String>,
    #[serde(skip)]
    pub category_attributes: HashMap<String, Vec<String>>,
}

impl Overlay for LabelOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.capture_base
    }
    fn set_capture_base(&mut self, said: &said::SelfAddressingIdentifier) {
        self.capture_base = Some(said.clone());
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn said(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.said
    }
    fn language(&self) -> Option<&Language> {
        Some(&self.language)
    }
    fn attributes(&self) -> Vec<&String> {
        self.attribute_labels.keys().collect::<Vec<&String>>()
    }
    /// Add an attribute to the Label Overlay
    /// TODO add assignment of attribute to category
    fn add(&mut self, attribute: &Attribute) {
        if let Some(labels) = &attribute.labels {
            if let Some(value) = labels.get(&self.language) {
                self.attribute_labels
                    .insert(attribute.name.clone(), value.to_string());
            }
        }
        if let Some(category_labels) = &attribute.category_labels {
            if let Some(value) = category_labels.get(&self.language) {
                self.category_labels
                    .insert(attribute.name.clone(), value.to_string());
            }
        }
    }
}

impl LabelOverlay {
    pub fn new(lang: Language) -> LabelOverlay {
        LabelOverlay {
            capture_base: None,
            said: None,
            overlay_type: "spec/overlays/label/1.0".to_string(),
            language: lang,
            attribute_labels: HashMap::new(),
            attribute_categories: vec![],
            category_labels: HashMap::new(),
            category_attributes: HashMap::new(),
        }
    }

    fn add_to_category(&mut self, categories: Vec<&str>, attribute: &Attribute) {
        let mut supercats: Vec<i32> = vec![];
        for (i, category) in categories.iter().enumerate() {
            let supercats_str: Vec<String> = supercats.iter().map(|c| c.to_string()).collect();
            let mut supercat = String::new();
            if !supercats_str.is_empty() {
                supercat = format!("-{}", supercats_str.join("-"))
            }
            let regex = regex::Regex::new(format!("^_cat{supercat}(-[0-9]*)_$").as_str()).unwrap();
            let mut acctual_cat_id = String::new();
            let mut category_exists = false;
            for (cat_id, cat_label) in self.category_labels.iter() {
                if cat_label == category && regex.is_match(cat_id) {
                    let cat_temp = cat_id.replace('_', "");
                    let mut temp = cat_temp.split('-').collect::<Vec<&str>>();
                    temp.remove(0);
                    supercats = temp.iter().map(|c| c.parse::<i32>().unwrap()).collect();
                    acctual_cat_id = cat_id.to_string();
                    category_exists = true;
                }
            }

            if !category_exists {
                let mut count = 0;
                for cat in self.attribute_categories.iter() {
                    if regex.is_match(cat.as_str()) {
                        count += 1;
                    }
                }
                acctual_cat_id = format!("_cat{}-{}_", supercat, count + 1);
                supercats.push(count + 1);
                self.category_labels
                    .insert(acctual_cat_id.clone(), category.to_string());
                self.attribute_categories.push(acctual_cat_id.clone());
                self.category_attributes
                    .insert(acctual_cat_id.clone(), vec![]);
            }

            if i + 1 == categories.len() {
                self.category_attributes
                    .get_mut(acctual_cat_id.as_str())
                    .unwrap()
                    .push(attribute.name.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_label_overlay() {
        let mut overlay = LabelOverlay::new(Language::Eng);
        let attr = cascade! {
            Attribute::new("attr1".to_string());
            ..set_label(Language::Pol, "Etykieta".to_string());
            ..set_label(Language::Eng, "Label".to_string());
            ..add_category_label(Language::Eng, "Category".to_string());
            ..add_category_label(Language::Pol, "Kategoria".to_string());
        };
        // even that attribute has 2 lagnuage only one attribute should be added to the overlay according to it's language
        overlay.add(&attr);

        assert_eq!(overlay.overlay_type, "spec/overlays/label/1.0");
        assert_eq!(overlay.language, Language::Eng);
        assert_eq!(overlay.attribute_labels.len(), 1);
        assert_eq!(overlay.category_labels.len(), 1);
    }
    #[test]
    fn resolve_categories_from_label() {
        let mut overlay = LabelOverlay::new(Language::Eng);
        let attr = cascade! {
            Attribute::new("attr1".to_string());
            ..set_label(Language::Pol, "Label 1".to_string());
            ..add_category_label(Language::Eng, "Cat 1".to_string());
        };
        overlay.add(&attr);
        let attr = cascade! {
            Attribute::new("attr2".to_string());
            ..set_label(Language::Pol, "Label 2".to_string());
            ..add_category_label(Language::Eng, "Cat 2".to_string());
        };
        overlay.add(&attr);

        assert_eq!(overlay.category_labels.len(), 2);
        assert!(overlay
            .attribute_categories
            .contains(&"_cat-1_".to_string()));
        assert!(overlay
            .attribute_categories
            .contains(&"_cat-2_".to_string()));

        assert!(overlay
            .category_labels
            .get(&"_cat-1_".to_string())
            .is_some());
        if let Some(cat1) = overlay.category_labels.get(&"_cat-1_".to_string()) {
            assert_eq!(*cat1, "Cat 1".to_string());
        }
        assert!(overlay
            .category_labels
            .get(&"_cat-2_".to_string())
            .is_some());
        if let Some(cat2) = overlay.category_labels.get(&"_cat-2_".to_string()) {
            assert_eq!(*cat2, "Cat 2".to_string());
        }

        assert!(overlay
            .category_attributes
            .get(&"_cat-1_".to_string())
            .is_some());
        if let Some(cat1_attrs) = overlay.category_attributes.get(&"_cat-1_".to_string()) {
            assert_eq!(cat1_attrs.len(), 1);
            assert!(cat1_attrs.contains(&"attr1".to_string()));
        }
        assert!(overlay
            .category_attributes
            .get(&"_cat-2_".to_string())
            .is_some());
        if let Some(cat2_attrs) = overlay.category_attributes.get(&"_cat-2_".to_string()) {
            assert_eq!(cat2_attrs.len(), 1);
            assert!(cat2_attrs.contains(&"attr2".to_string()));
        }
    }
}
