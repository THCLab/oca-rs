use crate::state::{attribute::Attribute, oca::Overlay};
use isolang::Language;
use oca_ast_semantics::ast::OverlayType;
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::any::Any;
use std::collections::HashMap;

pub type Framing = HashMap<String, FramingScope>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FramingScope {
    pub predicate_id: String,
    pub framing_justification: String,
    #[serde(skip)]
    pub frame_meta: HashMap<String, String>,
}

pub trait Framings {
    fn set_framing(&mut self, id: String, framing: Framing);
}

impl Framings for Attribute {
    fn set_framing(&mut self, id: String, framing: Framing) {
        match self.framings {
            Some(ref mut framings) => {
                if let Some(f) = framings.get_mut(&id) {
                    f.extend(framing);
                } else {
                    framings.insert(id, framing);
                }
            }
            None => {
                let mut framings = HashMap::new();
                framings.insert(id, framing);
                self.framings = Some(framings);
            }
        }
    }
}

pub fn serialize_metadata<S>(
    metadata: &HashMap<String, String>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use std::collections::BTreeMap;

    let mut ser = s.serialize_map(Some(metadata.len()))?;
    let sorted_metadata: BTreeMap<_, _> = metadata.iter().collect();
    for (k, v) in sorted_metadata {
        ser.serialize_entry(k, v)?;
    }
    ser.end()
}

pub fn serialize_framing<S>(
    attributes: &HashMap<String, Framing>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use std::collections::BTreeMap;

    let mut ser = s.serialize_map(Some(attributes.len()))?;
    let sorted_attributes: BTreeMap<_, _> = attributes.iter().collect();
    for (k, v) in sorted_attributes {
        let sorted_framings: BTreeMap<_, _> = v.iter().collect();
        ser.serialize_entry(k, &sorted_framings)?;
    }
    ser.end()
}

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct AttributeFramingOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    capture_base: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: OverlayType,
    #[serde(rename = "framing_metadata", serialize_with = "serialize_metadata")]
    pub metadata: HashMap<String, String>,
    #[serde(serialize_with = "serialize_framing")]
    pub attribute_framing: HashMap<String, Framing>,
}

impl Overlay for AttributeFramingOverlay {
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
    fn language(&self) -> Option<&Language> {
        None
    }
    fn attributes(&self) -> Vec<&String> {
        self.attribute_framing.keys().collect::<Vec<&String>>()
    }
    /// Add an attribute to the Label Overlay
    /// TODO add assignment of attribute to category
    fn add(&mut self, attribute: &Attribute) {
        if let Some(id) = self.metadata.get("frame_id") {
            if let Some(framing) = &attribute.framings {
                if let Some(value) = framing.get(id) {
                    self.attribute_framing
                        .insert(attribute.name.clone(), value.clone());

                    for framing_scope in value.values() {
                        for (k, v) in framing_scope.frame_meta.iter() {
                            self.metadata.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
        }
    }
}

impl AttributeFramingOverlay {
    pub fn new(id: String) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("frame_id".to_string(), id);
        Self {
            capture_base: None,
            said: None,
            overlay_type: OverlayType::AttributeFraming,
            metadata,
            attribute_framing: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_attribute_framing_overlay() {
        let mut overlay = AttributeFramingOverlay::new("frame_id".to_string());
        let mut loc1 = HashMap::new();
        loc1.insert(
            "http://loc.1".to_string(),
            FramingScope {
                predicate_id: "skos:exactMatch".to_string(),
                framing_justification: "semapv:ManualMappingCuration"
                    .to_string(),
                frame_meta: HashMap::new(),
            },
        );
        let mut loc2 = HashMap::new();
        loc2.insert(
            "http://loc.2".to_string(),
            FramingScope {
                predicate_id: "skos:exactMatch".to_string(),
                framing_justification: "semapv:ManualMappingCuration"
                    .to_string(),
                frame_meta: HashMap::new(),
            },
        );
        let attr = cascade! {
            Attribute::new("attr1".to_string());
            ..set_framing("frame_id".to_string(), loc1);
            ..set_framing("frame_id".to_string(), loc2);
        };
        // even that attribute has 2 lagnuage only one attribute should be added to the overlay according to it's language
        overlay.add(&attr);

        assert_eq!(overlay.overlay_type, OverlayType::AttributeFraming);
        assert_eq!(overlay.attribute_framing.len(), 1);
    }
}
