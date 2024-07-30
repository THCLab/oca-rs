use crate::state::oca::overlay::overlay;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{ser::SerializeMap, Serialize};

overlay!(Format, attribute_formats, format: String);
