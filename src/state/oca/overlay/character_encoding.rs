use crate::state::oca::overlay::overlay;
use crate::state::encoding::Encoding;
use serde::ser::SerializeStruct;

overlay!(CharacterEncoding, attribute_character_encoding, encoding: Encoding);
