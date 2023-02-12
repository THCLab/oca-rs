use crate::state::{
    attribute::Attribute,
    oca::Overlay,
};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use isolang::Language;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaOverlay {
    capture_base: String,
    #[serde(rename = "digest")]
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    #[serde(flatten)]
    pub attr_pairs: HashMap<String, String>,
}


// TODO: why Overlay implements fn which are not relevant for MetaOverlay?
impl Overlay for MetaOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &String {
        &self.capture_base
    }
    fn capture_base_mut(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn said(&self) -> &String {
        &self.said
    }
    fn said_mut(&mut self) -> &mut String {
        &mut self.said
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn language(&self) -> Option<&Language> {
        Some(&self.language)
    }
    fn attributes(&self) -> Vec<&String> {
        vec![]
    }

    fn add(&mut self, _attribute: &Attribute) {}
}
impl MetaOverlay {
    pub fn new(lang: Language, attr_pairs: HashMap<String, String>) -> Box<MetaOverlay> {
        Box::new(MetaOverlay {
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/meta/1.0".to_string(),
            language: lang,
            attr_pairs: attr_pairs,

        })
    }
}
