use indexmap::IndexMap;
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use said::version::SerializationInfo;
use serde::{Deserialize, Serialize};

#[derive(SAD, Serialize, Debug, Deserialize, Clone)]
#[version(protocol = "OCAT", major = 1, minor = 0)]
// #[said(format = "JSON")]
pub struct Transformation {
    #[said]
    #[serde(rename = "d")]
    pub said: Option<said::SelfAddressingIdentifier>,
    pub source: Option<String>,
    pub target: Option<String>,
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
            source: None,
            target: None,
            attributes: IndexMap::new(),
        }
    }

    pub fn set_source(&mut self, source: String) {
        self.source = Some(source);
    }
    pub fn set_target(&mut self, target: String) {
        self.target = Some(target);
    }

    pub fn rename(&mut self, attributes: IndexMap<String, String>) {
        attributes.into_iter().for_each(|(k, v)| {
            self.attributes.insert(k, v);
        });
    }

    pub fn link(&mut self, attributes: IndexMap<String, String>) {
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
