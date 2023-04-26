use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
struct LayoutConfigCss {
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct LayoutConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    css: Option<LayoutConfigCss>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ElementConfigCss {
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classes: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
struct ElementConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    css: Option<ElementConfigCss>,
    #[serde(skip_serializing_if = "Option::is_none")]
    widget: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<ElementConfig>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Element {
    #[serde(rename = "type")]
    element_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<ElementConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parts: Option<Vec<Part>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<LayoutConfig>,
    elements: Vec<Element>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_layouts: Option<BTreeMap<String, Layout>>,
}
