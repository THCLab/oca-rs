use crate::state::{attribute::Attribute, language::Language, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InformationOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    attr_information: BTreeMap<String, String>,
}

impl Overlay for InformationOverlay {
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
        self.attr_information.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(tr) = attribute.translations.get(&self.language) {
            if let Some(info) = &tr.information {
                self.attr_information
                    .insert(attribute.name.clone(), info.clone());
            }
        }
    }
}
impl InformationOverlay {
    pub fn new(lang: Language) -> Box<InformationOverlay> {
        Box::new(InformationOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overlays/information/1.0".to_string(),
            language: lang,
            attr_information: BTreeMap::new(),
        })
    }
}
