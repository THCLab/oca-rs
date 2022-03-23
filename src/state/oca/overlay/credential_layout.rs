use crate::state::{attribute::Attribute, oca::layout::credential::Layout, oca::Overlay};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CredentialLayoutOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    layout: Layout,
}

impl Overlay for CredentialLayoutOverlay {
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
impl CredentialLayoutOverlay {
    pub fn new(layout_str: String) -> Box<CredentialLayoutOverlay> {
        let yaml: Result<Layout, _> = serde_yaml::from_str(&layout_str);
        match yaml {
            Ok(layout) => Box::new(CredentialLayoutOverlay {
                capture_base: String::new(),
                overlay_type: "spec/overlays/credential_layout/1.0".to_string(),
                layout,
            }),
            Err(e) => panic!("{:#?}", e),
        }
    }
}
