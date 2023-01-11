use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormatOverlay {
    capture_base: String,
    #[serde(rename = "digest")]
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub attribute_formats: BTreeMap<String, String>,
}

impl Overlay for FormatOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn said(&self) -> &String {
        &self.said
    }
    fn said_mut(&mut self) -> &mut String {
        &mut self.said
    }
    fn attributes(&self) -> Vec<&String> {
        self.attribute_formats.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.format.is_some() {
            self.attribute_formats.insert(
                attribute.name.clone(),
                attribute.format.as_ref().unwrap().clone(),
            );
        }
    }
}
impl FormatOverlay {
    pub fn new() -> Box<FormatOverlay> {
        Box::new(FormatOverlay {
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/format/1.0".to_string(),
            attribute_formats: BTreeMap::new(),
        })
    }
}
