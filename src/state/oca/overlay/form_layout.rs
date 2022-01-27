use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormLayoutOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    layout: String,
}

impl Overlay for FormLayoutOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        vec![]
    }

    fn add(&mut self, _attribute: &Attribute) {}
}
impl FormLayoutOverlay {
    pub fn new(layout: String) -> Box<FormLayoutOverlay> {
        Box::new(FormLayoutOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overlays/form_layout/1.0".to_string(),
            layout,
        })
    }
}
