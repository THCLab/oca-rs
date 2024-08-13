use oca_bundle_semantics::state::oca::OCABundle;
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use said::version::SerializationInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum BundleElement {
    Mechanics(OCABundle),
    Transformation(transformation_file::state::Transformation),
}

#[derive(SAD, Serialize, Debug, Deserialize, Clone)]
#[version(protocol = "B", major = 1, minor = 0)]
pub struct Bundle {
    #[said]
    #[serde(rename = "d")]
    pub said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "m")]
    mechanics: Option<OCABundle>,
    #[serde(rename = "t")]
    transformations: Vec<transformation_file::state::Transformation>,
}

impl Bundle {
    pub fn new() -> Self {
        Self {
            said: None,
            mechanics: None,
            transformations: vec![],
        }
    }

    pub fn add(&mut self, element: BundleElement) {
        match element {
            BundleElement::Mechanics(mechanics) => self.add_mechanics(mechanics),
            BundleElement::Transformation(transformation) => self.add_transformation(transformation),
        }
    }

    fn add_mechanics(&mut self, mechanics: OCABundle) {
        self.mechanics = Some(mechanics);
    }

    fn add_transformation(&mut self, transformation: transformation_file::state::Transformation) {
        self.transformations.push(transformation);
    }

    pub fn fill_said(&mut self) {
        let code = HashFunctionCode::Blake3_256;
        let format = SerializationFormats::JSON;
        self.compute_digest(&code, &format);
    }
}

impl Default for Bundle {
    fn default() -> Self {
        Self::new()
    }
}

