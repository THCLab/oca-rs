use crate::state::oca::overlay::overlay;
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{ser::SerializeMap, Serialize};

overlay!(Conformance, attribute_conformance, conformance: String);
