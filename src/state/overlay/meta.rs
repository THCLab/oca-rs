use crate::state::Overlay;
use crate::state::{Attribute, BundleTranslation, Language};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    name: String,
    descritpion: String,
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

    fn add(&mut self, _attribute: &Attribute) {}
}
impl MetaOverlay {
    pub fn new(lang: &Language, bundle_tr: &BundleTranslation) -> Box<MetaOverlay> {
        Box::new(MetaOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/meta/1.0".to_string(),
            language: *lang,
            name: bundle_tr.name.clone(),
            descritpion: bundle_tr.descritpion.clone(),
        })
    }
}
