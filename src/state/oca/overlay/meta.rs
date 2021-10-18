use crate::state::{
    attribute::Attribute,
    language::Language,
    oca::{OCATranslation, Overlay},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    name: String,
    description: String,
}

impl Overlay for MetaOverlay {
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
        vec![]
    }

    fn add(&mut self, _attribute: &Attribute) {}
}
impl MetaOverlay {
    pub fn new(lang: Language, oca_tr: &OCATranslation) -> Box<MetaOverlay> {
        Box::new(MetaOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/meta/1.0".to_string(),
            language: lang,
            name: oca_tr.name.as_ref().unwrap_or(&"".to_string()).clone(),
            description: oca_tr
                .description
                .as_ref()
                .unwrap_or(&"".to_string())
                .clone(),
        })
    }
}
