use crate::state::oca::overlay::overlay;
use serde::{Serialize, ser::SerializeMap};
use said::{sad::SAD, sad::SerializationFormats};

overlay!(Conformance, attribute_conformance, conformance: String);
