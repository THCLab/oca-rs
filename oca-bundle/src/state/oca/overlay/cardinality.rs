use crate::state::oca::overlay::overlay;
use serde::{Serialize, ser::SerializeMap};
use said::{sad::SAD, sad::SerializationFormats};

overlay!(Cardinality, attribute_cardinality, cardinality: String);
