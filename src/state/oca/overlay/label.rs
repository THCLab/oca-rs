use crate::state::{attribute::Attribute, language::Language, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    attr_labels: BTreeMap<String, String>,
    attr_categories: Vec<String>,
    cat_labels: BTreeMap<String, String>,
    cat_attributes: BTreeMap<String, Vec<String>>,
}

impl Overlay for LabelOverlay {
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
        self.attr_labels.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(tr) = attribute.translations.get(&self.language) {
            if let Some(label) = &tr.label {
                self.attr_labels
                    .insert(attribute.name.clone(), label.clone());
            }
            self.cat_attributes
                .get_mut("_cat-1_")
                .unwrap()
                .push(attribute.name.clone());
        }
    }
}
impl LabelOverlay {
    pub fn new(lang: Language) -> Box<LabelOverlay> {
        let mut cat_labels = BTreeMap::new();
        cat_labels.insert(String::from("_cat-1_"), String::from("Category 1"));
        let mut cat_attributes = BTreeMap::new();
        cat_attributes.insert(String::from("_cat-1_"), vec![]);
        Box::new(LabelOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/label/1.0".to_string(),
            language: lang,
            attr_labels: BTreeMap::new(),
            attr_categories: vec![String::from("_cat-1_")],
            cat_labels,
            cat_attributes,
        })
    }
}
