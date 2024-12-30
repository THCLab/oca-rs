use crate::state::{attribute::Attribute, oca::Overlay};
use oca_ast_semantics::ast::OverlayType;
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::any::Any;
use std::collections::HashMap;

pub trait Links {
    fn set_link(&mut self, t: String, link: String);
}

impl Links for Attribute {
    fn set_link(&mut self, t: String, link: String) {
        if let Some(links) = &mut self.links {
            links.insert(t, link);
        } else {
            let mut links = HashMap::new();
            links.insert(t, link);
            self.links = Some(links);
        }
    }
}

pub fn serialize_attributes<S>(
    attributes: &HashMap<String, String>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use std::collections::BTreeMap;

    let mut ser = s.serialize_map(Some(attributes.len()))?;
    let sorted_attributes: BTreeMap<_, _> = attributes.iter().collect();
    for (k, v) in sorted_attributes {
        ser.serialize_entry(k, v)?;
    }
    ser.end()
}

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct LinkOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    capture_base: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: OverlayType,
    pub target_bundle: String,
    #[serde(serialize_with = "serialize_attributes")]
    pub attribute_mapping: HashMap<String, String>,
}

impl Overlay for LinkOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.capture_base
    }
    fn set_capture_base(&mut self, said: &said::SelfAddressingIdentifier) {
        self.capture_base = Some(said.clone());
    }
    fn overlay_type(&self) -> &OverlayType {
        &self.overlay_type
    }
    fn said(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.said
    }
    fn attributes(&self) -> Vec<&String> {
        self.attribute_mapping.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(links) = &attribute.links {
            if let Some(value) = links.get(&self.target_bundle) {
                self.attribute_mapping
                    .insert(attribute.name.clone(), value.to_string());
            }
        }
    }
}
impl LinkOverlay {
    pub fn new(t: String) -> Self {
        Self {
            capture_base: None,
            said: None,
            overlay_type: OverlayType::Link,
            target_bundle: t,
            attribute_mapping: HashMap::new(),
        }
    }
}
