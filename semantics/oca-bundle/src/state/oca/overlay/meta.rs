use crate::state::{attribute::Attribute, oca::OCABox, oca::Overlay};
use isolang::Language;
use oca_ast::ast::OverlayType;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::any::Any;
use std::collections::HashMap;

pub trait Metas {
    fn add_meta(&mut self, language: Language, key: String, value: String);
}

impl Metas for OCABox {
    fn add_meta(&mut self, l: Language, key: String, value: String) {
        match &mut self.meta {
            Some(ref mut meta) => match meta.get_mut(&l) {
                Some(attr_pairs) => {
                    attr_pairs.insert(key, value);
                }
                None => {
                    let mut attr_pairs = HashMap::new();
                    attr_pairs.insert(key, value);
                    meta.insert(l, attr_pairs);
                }
            },
            None => {
                let meta = HashMap::new();
                self.meta = Some(meta);
                self.add_meta(l, key, value);
            }
        };
    }
}

pub fn serialize_attributes<S>(
    attributes: &HashMap<String, String>,
    s: S,
) -> Result<S::Ok, S::Error>
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

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct MetaOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    pub language: Language,
    #[serde(rename = "type")]
    overlay_type: OverlayType,
    capture_base: Option<said::SelfAddressingIdentifier>,
    #[serde(flatten, serialize_with = "serialize_attributes")]
    pub attr_pairs: HashMap<String, String>,
}

// TODO: why Overlay implements fn which are not relevant for MetaOverlay?
impl Overlay for MetaOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.capture_base
    }
    fn set_capture_base(&mut self, said: &said::SelfAddressingIdentifier) {
        self.capture_base = Some(said.clone());
    }
    fn said(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.said
    }
    fn overlay_type(&self) -> &OverlayType {
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
            capture_base: None,
            said: None,
            overlay_type: OverlayType::Meta,
            language: lang,
            attr_pairs,
        }
    }
}
