use crate::state::oca::overlay::overlay;
use serde::ser::SerializeStruct;

overlay!(EntryCode, attribute_entry_codes, entry_codes: crate::state::entry_codes::EntryCodes);
