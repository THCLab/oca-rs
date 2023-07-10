use serde::{Deserialize, Serialize, ser::SerializeMap};
use std::collections::HashMap;

impl Serialize for EntriesElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        use std::collections::BTreeMap;

        match self {
            Self::Sai(sai) => serializer.serialize_str(sai),
            Self::Object(entries) => {
                let sorted_entries: BTreeMap<_, _> = entries.iter().collect();
                let mut map = serializer.serialize_map(Some(entries.len()))?;
                for (k, v) in sorted_entries {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EntriesElement {
    Sai(String),
    Object(HashMap<String, String>),
}
