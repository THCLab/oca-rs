use indexmap::IndexMap;
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use said::version::SerializationInfo;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(SAD, Serialize, Debug, Deserialize, Clone)]
#[version(protocol = "T", major = 1, minor = 0)]
// #[said(format = "JSON")]
pub struct Transformation {
    #[said]
    #[serde(rename = "d")]
    pub said: Option<said::SelfAddressingIdentifier>,
    pub attributes: IndexMap<String, String>,
}

impl Default for Transformation {
    fn default() -> Self {
        Self::new()
    }
}

impl Transformation {
    pub fn new() -> Self {
        Self {
            said: None,
            attributes: IndexMap::new(),
        }
    }

    pub fn rename(&mut self, attributes: IndexMap<String, String>) {
        attributes.into_iter().for_each(|(k, v)| {
            self.attributes.insert(k, v);
        });
    }

    pub fn fill_said(&mut self) {
        let code = HashFunctionCode::Blake3_256;
        let format = SerializationFormats::JSON;
        self.compute_digest(&code, &format);
    }
}
