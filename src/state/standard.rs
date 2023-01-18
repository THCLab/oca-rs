use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone)]
pub struct Standard {
    value: String,
}

impl Standard {
    pub fn new(value: String) -> Self {
        Self {
            value: value.to_lowercase(),
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
            Ok(Standard::new(value))
        } else {
            Err(serde::de::Error::custom("standard must be a string"))
        }
    }
}
