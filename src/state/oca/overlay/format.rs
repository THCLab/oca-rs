use crate::state::oca::overlay::overlay;
use serde::ser::SerializeStruct;

overlay!(Format, attribute_formats, format: String);
