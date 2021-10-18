use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    attr_units: BTreeMap<String, String>,
}

impl Overlay for UnitOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        self.attr_units.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.unit.is_some() {
            self.attr_units.insert(
                attribute.name.clone(),
                attribute.unit.as_ref().unwrap().clone(),
            );
        }
    }
}
impl UnitOverlay {
    pub fn new() -> Box<UnitOverlay> {
        Box::new(UnitOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/unit/1.0".to_string(),
            attr_units: BTreeMap::new(),
        })
    }
}