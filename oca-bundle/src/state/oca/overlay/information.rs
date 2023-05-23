use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};
use std::any::Any;
use std::collections::HashMap;
use isolang::Language;
use said::{sad::SAD, sad::SerializationFormats, derivation::HashFunctionCode};

pub trait Information {
    fn set_information(&mut self, l: Language, information: String);
}

impl Information for Attribute {
    fn set_information(&mut self, l: Language, information: String) {
        if let Some(informations) = &mut self.informations {
            informations.insert(l, information);
        } else {
            let mut informations = HashMap::new();
            informations.insert(l, information);
            self.informations = Some(informations);
        }
    }
}

pub fn serialize_attributes<S>(attributes: &HashMap<String, String>, s: S) -> Result<S::Ok, S::Error>
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
pub struct InformationOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    language: Language,
    #[serde(rename = "type")]
    overlay_type: String,
    capture_base: Option<said::SelfAddressingIdentifier>,
    #[serde(serialize_with = "serialize_attributes")]
    pub attribute_information: HashMap<String, String>,
}

impl Overlay for InformationOverlay {
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
    fn language(&self) -> Option<&Language> {
        Some(&self.language)
    }
    fn attributes(&self) -> Vec<&String> {
        self.attribute_information.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(informations) = &attribute.informations {
            if let Some(value) = informations.get(&self.language) {
                self.attribute_information
                    .insert(attribute.name.clone(), value.to_string());
            }
        }
    }
}
impl InformationOverlay {
    pub fn new(lang: Language) -> InformationOverlay {
        InformationOverlay {
            capture_base: None,
            said: None,
            overlay_type: "spec/overlays/information/1.0".to_string(),
            language: lang,
            attribute_information: HashMap::new(),
        }
    }
}
