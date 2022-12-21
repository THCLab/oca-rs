use crate::state::{attribute::Attribute, oca::layout::form::Layout, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Serialize, Deserialize, Debug)]
pub struct FormLayoutOverlay {
    capture_base: String,
    #[serde(rename = "digest")]
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub layout: Layout,
}

impl Overlay for FormLayoutOverlay {
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
        vec![]
    }

    fn add(&mut self, _attribute: &Attribute) {}
}
impl FormLayoutOverlay {
    pub fn new(layout_str: String) -> Box<FormLayoutOverlay> {
        let yaml: Result<Layout, _> = serde_yaml::from_str(&layout_str);
        match yaml {
            Ok(layout) => Box::new(FormLayoutOverlay {
                capture_base: String::new(),
                said: String::from("############################################"),
                overlay_type: "spec/overlays/form_layout/1.0".to_string(),
                layout,
            }),
            Err(e) => panic!("{:#?}", e),
        }
    }
}
