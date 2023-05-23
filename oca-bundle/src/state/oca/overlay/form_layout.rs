use crate::state::oca::OCABox;
use crate::state::{attribute::Attribute, oca::layout::form::Layout, oca::Overlay};
use serde::{Deserialize, Serialize};
use std::any::Any;
use said::{sad::SAD, sad::SerializationFormats, derivation::HashFunctionCode};

pub trait FormLayouts {
    fn add_form_layout(&mut self, layout_str: String);
}

impl FormLayouts for OCABox {
    fn add_form_layout(&mut self, layout_str: String) {
        let yaml: Result<Layout, _> = serde_yaml::from_str(&layout_str);
        match yaml {
            Ok(layout) => {
                if let Some(layouts) = self.form_layouts.as_mut() {
                    layouts.push(layout);
                } else {
                    self.form_layouts = Some(vec![layout]);
                }
            }
            Err(e) => panic!("{:#?}", e),
        }
    }
}

#[derive(SAD, Serialize, Deserialize, Clone, Debug)]
pub struct FormLayoutOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: String,
    capture_base: Option<said::SelfAddressingIdentifier>,
    pub layout: Layout,
}

impl Overlay for FormLayoutOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.capture_base
    }
    fn set_capture_base(&mut self, said: &said::SelfAddressingIdentifier) {
        self.capture_base = Some(said.clone());
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn said(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.said
    }
    fn attributes(&self) -> Vec<&String> {
        vec![]
    }

    fn add(&mut self, _attribute: &Attribute) {}
}

impl FormLayoutOverlay {
    pub fn new(layout: Layout) -> Self {
        Self {
            capture_base: None,
            said: None,
            overlay_type: "spec/overlays/form_layout/1.0".to_string(),
            layout,
        }
    }
}
