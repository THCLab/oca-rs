use crate::state::oca::OCABox;
use crate::state::{attribute::Attribute, oca::layout::form::Layout, oca::Overlay};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use std::any::Any;

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

impl Serialize for FormLayoutOverlay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FormLayoutOverlay", 4)?;
        state.serialize_field("said", &self.said)?;
        state.serialize_field("type", &self.overlay_type)?;
        state.serialize_field("capture_base", &self.capture_base)?;
        state.serialize_field("layout", &self.layout)?;
        state.end()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct FormLayoutOverlay {
    capture_base: String,
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub layout: Layout,
}

impl Overlay for FormLayoutOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &String {
        &self.capture_base
    }
    fn capture_base_mut(&mut self) -> &mut String {
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
    pub fn new(layout: Layout) -> Self {
        Self {
            capture_base: String::new(),
            said: String::from("############################################"),
            overlay_type: "spec/overlays/form_layout/1.0".to_string(),
            layout,
        }
    }
}
