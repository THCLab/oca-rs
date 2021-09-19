use crate::state::{Attribute, AttributeType};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct CaptureBase {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub classification: String,
    #[serde(serialize_with = "ordered_attributes")]
    pub attributes: HashMap<String, AttributeType>,
    pub pii: Vec<String>,
}

impl CaptureBase {
    pub fn new() -> CaptureBase {
        CaptureBase {
            schema_type: String::from("spec/capture_base/1.0"),
            classification: String::from("classification"),
            attributes: HashMap::new(),
            pii: Vec::new(),
        }
    }

    pub fn add(&mut self, attribute: &Attribute) {
        self.attributes
            .insert(attribute.name.clone(), attribute.attr_type);
        if attribute.is_pii {
            self.pii.push(attribute.name.clone());
        }
    }
}

fn ordered_attributes<S>(
    value: &HashMap<String, AttributeType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}
