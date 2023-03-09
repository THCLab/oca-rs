use crate::state::{
    attribute::Attribute,
    oca::Overlay,
    oca::OCABox,
};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};
use std::any::Any;
use std::collections::HashMap;
use isolang::Language;

pub trait Metas {
    fn add_meta(&mut self, language: Language, key: String, value: String);
}

impl Metas for OCABox {
    fn add_meta(&mut self, l: Language, key: String, value: String) {
        match &mut self.meta {
            Some(ref mut meta) => {
                match meta.get_mut(&l) {
                    Some(attr_pairs) => {
                        attr_pairs.insert(key, value);
                    },
                    None => {
                        let mut attr_pairs = HashMap::new();
                        attr_pairs.insert(key, value);
                        meta.insert(l, attr_pairs);
                    }
                }
            }
            None => {
                let meta = HashMap::new();
                self.meta = Some(meta);
                self.add_meta(l, key, value);
            }
        };
    }
}

impl Serialize for MetaOverlay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use std::collections::BTreeMap;

        let mut state = serializer.serialize_map(Some(4 + self.attr_pairs.len()))?;

        state.serialize_entry("said", &self.said)?;
        state.serialize_entry("language", &self.language)?;
        state.serialize_entry("type", &self.overlay_type)?;
        state.serialize_entry("capture_base", &self.capture_base)?;

        let sorted_attr_pairs: BTreeMap<_, _> = self.attr_pairs.iter().collect();
        for (k, v) in sorted_attr_pairs.iter() {
            state.serialize_entry(k, v)?;
        }

        state.end()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct MetaOverlay {
    capture_base: String,
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
    pub fn new(lang: Language, attr_pairs: HashMap<String, String>) -> Self {
        Self {
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/meta/1.0".to_string(),
            language: lang,
            attr_pairs,
        }
    }
}
