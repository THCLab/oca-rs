use crate::state::{attribute::Attribute, oca::Overlay};
use oca_ast::ast::OverlayType;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

pub trait Conditionals {
    fn set_condition(&mut self, condition: String);
}

impl Conditionals for Attribute {
    fn set_condition(&mut self, condition: String) {
        let re = regex::Regex::new(r"\$\{([^}]*)\}").unwrap();
        let mut dependencies = Vec::new();

        let cond = re
            .replace_all(&condition, |caps: &regex::Captures| {
                let i = dependencies
                    .iter()
                    .position(|d| d == &caps[1])
                    .unwrap_or_else(|| {
                        dependencies.push(caps[1].to_string());
                        dependencies.len() - 1
                    });
                format!("${{{}}}", i)
            })
            .to_string();

        self.condition = Some(cond);
        self.dependencies = Some(dependencies);
    }
}

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: OverlayType,
    capture_base: Option<said::SelfAddressingIdentifier>,
    pub attribute_conditions: BTreeMap<String, String>,
    pub attribute_dependencies: BTreeMap<String, Vec<String>>,
}

impl Overlay for ConditionalOverlay {
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
        self.attribute_conditions.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.condition.is_some() {
            self.attribute_conditions.insert(
                attribute.name.clone(),
                attribute.condition.as_ref().unwrap().clone(),
            );
        }
        if attribute.dependencies.is_some() {
            self.attribute_dependencies.insert(
                attribute.name.clone(),
                attribute.dependencies.as_ref().unwrap().clone(),
            );
        }
    }
}
impl ConditionalOverlay {
    pub fn new() -> Self {
        Self {
            capture_base: None,
            said: None,
            overlay_type: OverlayType::Conditional,
            attribute_conditions: BTreeMap::new(),
            attribute_dependencies: BTreeMap::new(),
        }
    }
}

impl Default for ConditionalOverlay {
    fn default() -> Self {
        Self::new()
    }
}
