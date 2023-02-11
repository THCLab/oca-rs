use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Standard {
    value: String,
}

impl Standard {
    pub fn new(value: String) -> Self {
        Self {
            value: value.to_lowercase(),
        }
    }

    fn regexes(nid: &str) -> Option<Regex> {
        lazy_static! {
            static ref REGEXES: HashMap<String, String> = {
                let standards_str = include_str!("../../config/standards.yml");
                serde_yaml::from_str(standards_str).unwrap()
            };
        }
        match REGEXES.get(nid) {
            Some(re_str) => regex::Regex::new(re_str).ok(),
            None => None,
        }
    }

    pub fn validate(&self) -> Result<&Self, String> {
        let urn = urn::Urn::from_str(self.value.as_ref()).map_err(|e| e.to_string())?;
        match Self::regexes(urn.nid()) {
            Some(regex) => {
                if regex.is_match(urn.nss()) {
                    Ok(self)
                } else {
                    Err(format!(
                        "{} nss is invalid for {} namespace",
                        urn.nss(),
                        urn.nid()
                    ))
                }
            }
            None => Err(format!("{} namespace is unsupported", urn.nid())),
        }
    }
}

impl Serialize for Standard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value.as_str())
    }
}

impl<'de> Deserialize<'de> for Standard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let de_standard = serde_value::Value::deserialize(deserializer)?;

        if let serde_value::Value::String(value) = de_standard {
            Ok(Standard::new(value)
                .validate()
                .map_err(serde::de::Error::custom)?
                .clone())
        } else {
            Err(serde::de::Error::custom("standard must be a string"))
        }
    }
}
