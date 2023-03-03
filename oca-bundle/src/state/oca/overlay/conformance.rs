use crate::state::oca::overlay::overlay;
use serde::ser::SerializeStruct;

overlay!(Conformance, attribute_conformance, conformance: String);
