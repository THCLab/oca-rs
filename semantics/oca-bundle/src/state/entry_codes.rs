use indexmap::IndexMap;
use serde::{ser::{SerializeSeq, SerializeMap}, Deserialize, Serialize};

impl Serialize for EntryCodes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Sai(sai) => serializer.serialize_str(sai),
            Self::Array(codes) => {
                let mut seq = serializer.serialize_seq(Some(codes.len()))?;
                let mut sorted_codes = codes.clone();
                sorted_codes.sort();
                for code in sorted_codes {
                    seq.serialize_element(&code)?;
                }
                seq.end()
            }
            Self::Object(codes) => {
                let mut map = serializer.serialize_map(Some(codes.len()))?;
                let sorted_codes: IndexMap<_, _> = codes.iter().collect();
                for (k, v) in sorted_codes {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EntryCodes {
    Sai(String),
    Array(Vec<String>),
    Object(IndexMap<String, Vec<String>>),
}
