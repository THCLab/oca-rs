use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EntriesElement {
    Sai(String),
    Object(BTreeMap<String, String>),
}
