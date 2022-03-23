use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
struct LayoutConfigCss {
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct LayoutConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    css: Option<LayoutConfigCss>,
}
#[derive(Debug, Serialize, Deserialize)]
struct PageConfigCss {
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_image: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct PageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    css: Option<PageConfigCss>,
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
}
#[derive(Debug, Serialize, Deserialize)]
struct Element {
    #[serde(rename = "type")]
    element_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<ElementConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elements: Option<Vec<Element>>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Page {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<PageConfig>,
    elements: Vec<Element>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<LayoutConfig>,
    pages: Vec<Page>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<BTreeMap<String, BTreeMap<String, String>>>,
}
