use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EntriesElement {
    Sai(String),
    Object(HashMap<String, String>),
}
