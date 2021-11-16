use crate::state::{attribute::Attribute, encoding::Encoding, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterEncodingOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    default_character_encoding: Encoding,
    attr_character_encoding: BTreeMap<String, Encoding>,
}

impl Overlay for CharacterEncodingOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        self.attr_character_encoding
            .keys()
            .collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        self.attr_character_encoding
            .insert(attribute.name.clone(), attribute.encoding.unwrap());
    }
}
impl CharacterEncodingOverlay {
    pub fn new(encoding: &Encoding) -> Box<CharacterEncodingOverlay> {
        Box::new(CharacterEncodingOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overlays/character_encoding/1.0".to_string(),
            default_character_encoding: *encoding,
            attr_character_encoding: BTreeMap::new(),
        })
    }
}
