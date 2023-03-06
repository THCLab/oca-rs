use crate::state::oca::overlay::overlay;
use serde::ser::SerializeStruct;

overlay!(Cardinality, attribute_cardinality, cardinality: String);
