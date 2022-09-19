use crate::state::{attribute::Attribute, language::Language, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    pub attribute_labels: BTreeMap<String, String>,
    pub attribute_categories: Vec<String>,
    pub category_labels: BTreeMap<String, String>,
    #[serde(skip)]
    pub category_attributes: BTreeMap<String, Vec<String>>,
}

impl Overlay for LabelOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn language(&self) -> Option<&Language> {
        Some(&self.language)
    }
    fn attributes(&self) -> Vec<&String> {
        self.attribute_labels.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(tr) = attribute.translations.get(&self.language) {
            if let Some(value) = &tr.label {
                let mut splitted = value.split('|').collect::<Vec<&str>>();
                let label = splitted.pop().unwrap().to_string();
                self.attribute_labels.insert(attribute.name.clone(), label);
                self.add_to_category(splitted, attribute);
            }
        }
    }
}

impl LabelOverlay {
    pub fn new(lang: Language) -> Box<LabelOverlay> {
        Box::new(LabelOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overlays/label/1.0".to_string(),
            language: lang,
            attribute_labels: BTreeMap::new(),
            attribute_categories: vec![],
            category_labels: BTreeMap::new(),
            category_attributes: BTreeMap::new(),
        })
    }

    fn add_to_category(&mut self, categories: Vec<&str>, attribute: &Attribute) {
        let mut supercats: Vec<i32> = vec![];
        for (i, category) in categories.iter().enumerate() {
            let supercats_str: Vec<String> = supercats.iter().map(|c| c.to_string()).collect();
            let mut supercat = String::new();
            if !supercats_str.is_empty() {
                supercat = format!("-{}", supercats_str.join("-"))
            }
            let regex =
                regex::Regex::new(format!("^_cat{}(-[0-9]*)_$", supercat).as_str()).unwrap();
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
                self.category_attributes.insert(acctual_cat_id.clone(), vec![]);
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
    use crate::state::attribute::{AttributeBuilder, AttributeType};
    use maplit::hashmap;

    #[test]
    fn resolve_categories_from_label() {
        let mut overlay = LabelOverlay::new("En".to_string());
        overlay.add(
            &AttributeBuilder::new("attr1".to_string(), AttributeType::Text)
                .add_label(hashmap! {
                    "En".to_string() => "Cat 1|label 1".to_string()
                })
                .build(),
        );
        overlay.add(
            &AttributeBuilder::new("attr2".to_string(), AttributeType::Text)
                .add_label(hashmap! {
                    "En".to_string() => "Cat 2|label 2".to_string()
                })
                .build(),
        );

        assert_eq!(overlay.attribute_categories.len(), 2);
        assert!(overlay.attribute_categories.contains(&"_cat-1_".to_string()));
        assert!(overlay.attribute_categories.contains(&"_cat-2_".to_string()));

        assert!(overlay.category_labels.get(&"_cat-1_".to_string()).is_some());
        if let Some(cat1) = overlay.category_labels.get(&"_cat-1_".to_string()) {
            assert_eq!(*cat1, "Cat 1".to_string());
        }
        assert!(overlay.category_labels.get(&"_cat-2_".to_string()).is_some());
        if let Some(cat2) = overlay.category_labels.get(&"_cat-2_".to_string()) {
            assert_eq!(*cat2, "Cat 2".to_string());
        }

        assert!(overlay.category_attributes.get(&"_cat-1_".to_string()).is_some());
        if let Some(cat1_attrs) = overlay.category_attributes.get(&"_cat-1_".to_string()) {
            assert_eq!(cat1_attrs.len(), 1);
            assert!(cat1_attrs.contains(&"attr1".to_string()));
        }
        assert!(overlay.category_attributes.get(&"_cat-2_".to_string()).is_some());
        if let Some(cat2_attrs) = overlay.category_attributes.get(&"_cat-2_".to_string()) {
            assert_eq!(cat2_attrs.len(), 1);
            assert!(cat2_attrs.contains(&"attr2".to_string()));
        }
    }
}
