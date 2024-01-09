use crate::state::oca::OCABox;
use crate::state::{attribute::Attribute, oca::layout::credential::Layout, oca::Overlay};
use oca_ast::ast::OverlayType;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{Deserialize, Serialize};
use std::any::Any;

pub trait CredentialLayouts {
    fn add_credential_layout(&mut self, layout_str: String);
}

impl CredentialLayouts for OCABox {
    fn add_credential_layout(&mut self, layout_str: String) {
        let yaml: Result<Layout, _> = serde_yaml::from_str(&layout_str);
        match yaml {
            Ok(layout) => {
                if let Some(layouts) = self.credential_layouts.as_mut() {
                    layouts.push(layout);
                } else {
                    self.credential_layouts = Some(vec![layout]);
                }
            }
            Err(e) => panic!("{:#?}", e),
        }
    }
}

#[derive(SAD, Serialize, Deserialize, Clone, Debug)]
pub struct CredentialLayoutOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: OverlayType,
    capture_base: Option<said::SelfAddressingIdentifier>,
    pub layout: Layout,
}

impl Overlay for CredentialLayoutOverlay {
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
    fn attributes(&self) -> Vec<&String> {
        vec![]
    }

    fn add(&mut self, _attribute: &Attribute) {}
}

impl CredentialLayoutOverlay {
    pub fn new(layout: Layout) -> Self {
        Self {
            capture_base: None,
            said: None,
            overlay_type: OverlayType::Layout,
            layout,
        }
    }
}
