use crate::state::standard::Standard;
use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use said::{sad::SAD, sad::SerializationFormats, derivation::HashFunctionCode};

pub(crate) trait StandardAttribute {
    fn add_standard(&mut self, standard: Standard) -> ();
}

impl StandardAttribute for Attribute {
    fn add_standard(&mut self, standard: Standard) {
        match self.standards {
            Some(ref mut standards) => {
                standards.push(standard);
            }
            None => {
                self.standards = Some(vec![standard]);
            }
        }
    }
}

#[derive(SAD, Serialize, Deserialize, Clone)]
pub struct StandardOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: String,
    capture_base: Option<said::SelfAddressingIdentifier>,
    pub attribute_standards: HashMap<String, Standard>,
}

impl Overlay for StandardOverlay {
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
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        self.attribute_standards.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(standard) = &attribute.standards {
            // TODO find out how to pick right standard if there are more than one
            self.attribute_standards
                .insert(attribute.name.clone(), standard[0].clone());
        }
    }
}
impl StandardOverlay {
    pub fn new() -> Box<StandardOverlay> {
        Box::new(StandardOverlay {
            capture_base: None,
            said: None,
            overlay_type: "spec/overlays/standard/1.0".to_string(),
            attribute_standards: HashMap::new(),
        })
    }
}
