use crate::state::standard::Standard;
use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::{BTreeMap, HashMap};


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

#[derive(Serialize, Deserialize, Clone)]
pub struct StandardOverlay {
    capture_base: String,
    #[serde(rename = "digest")]
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub attribute_standards: HashMap<String, Standard>,
}

impl Overlay for StandardOverlay {
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
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/standard/1.0".to_string(),
            attribute_standards: HashMap::new(),
        })
    }
}
