use crate::state::oca::overlay::overlay;
use serde::{Serialize, ser::SerializeMap};
use said::{sad::SAD, sad::SerializationFormats};

overlay!(EntryCode, attribute_entry_codes, entry_codes: crate::state::entry_codes::EntryCodes);
