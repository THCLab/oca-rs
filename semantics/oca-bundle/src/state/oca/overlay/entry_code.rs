use crate::state::oca::overlay::overlay;
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{ser::SerializeMap, Serialize};

overlay!(EntryCode, attribute_entry_codes, entry_codes: crate::state::entry_codes::EntryCodes);
