use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EntryCodes {
    Sai(String),
    Array(Vec<String>),
}
