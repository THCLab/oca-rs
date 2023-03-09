use crate::state::oca::OCABox;
use crate::state::{attribute::Attribute, oca::layout::credential::Layout, oca::Overlay};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
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

impl Serialize for CredentialLayoutOverlay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CredentialLayoutOverlay", 4)?;
        state.serialize_field("said", &self.said)?;
        state.serialize_field("type", &self.overlay_type)?;
        state.serialize_field("capture_base", &self.capture_base)?;
        state.serialize_field("layout", &self.layout)?;
        state.end()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct CredentialLayoutOverlay {
    capture_base: String,
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub layout: Layout,
}

impl Overlay for CredentialLayoutOverlay {
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
        vec![]
    }

    fn add(&mut self, _attribute: &Attribute) {}
}

impl CredentialLayoutOverlay {
    pub fn new(layout: Layout) -> Self {
        Self {
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/credential_layout/1.0".to_string(),
            layout,
        }
    }
}
