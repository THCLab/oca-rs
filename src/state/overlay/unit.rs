use crate::state::Attribute;
use crate::state::Overlay;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    attr_units: HashMap<String, String>,
}

impl Overlay for UnitOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
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
            attr_units: HashMap::new(),
        })
    }
}
