use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bundle {
    pub capture_base: CaptureBase,
    pub overlays: Vec<Overlays>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaptureBase {
    #[serde(alias = "type")]
    schema_type: String,
    classification: String,
    attributes: HashMap<String, AttributeType>,
    pii: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AttributeType {
    Text,
    Number,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Overlays {}
